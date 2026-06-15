use super::support::*;

#[test]
fn test_start_refuses_shared_section_diagnostic() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "create", "Malformed section work"]);
    assert!(success, "issue create failed: {stderr}");
    assert!(stdout.contains("Created issue atelier-"));
    let issue_id = issue_id_by_title(dir.path(), "Malformed section work");
    migrate_default_issue_workflow(dir.path());
    commit_all(dir.path(), "valid issue");

    let issue_path = dir
        .path()
        .join(".atelier/issues")
        .join(format!("{issue_id}.md"));
    let markdown = std::fs::read_to_string(&issue_path).unwrap();
    let malformed = markdown.replace("\n## Outcome\n\nOutcome was not specified.\n", "\n");
    std::fs::write(&issue_path, malformed).unwrap();
    commit_all(dir.path(), "malformed issue section");

    let (lint_success, lint_stdout, lint_stderr) = run_atelier(dir.path(), &["lint"]);
    assert!(!lint_success, "lint should report malformed issue sections");
    let lint_transcript = format!("{lint_stdout}\n{lint_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            lint_transcript.contains(needle),
            "lint diagnostic missing {needle:?}: {lint_transcript}"
        );
    }

    let (start_success, start_stdout, start_stderr) =
        run_atelier(dir.path(), &["start", &issue_id]);
    assert!(
        !start_success,
        "start should refuse malformed issue sections"
    );
    let start_transcript = format!("{start_stdout}\n{start_stderr}");
    for needle in [
        "Missing required issue body section 'Outcome'",
        &issue_id,
        "section Outcome",
        ".atelier/issues/",
    ] {
        assert!(
            start_transcript.contains(needle),
            "start diagnostic missing {needle:?}: {start_transcript}"
        );
    }
}

#[test]
fn test_issue_type_is_canonical_not_label_derived() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let (success, stdout, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Typed issue",
            "--issue-type",
            "validation",
            "--label",
            "epic",
        ],
    );
    assert!(success, "create failed: {stderr}");
    assert!(stdout.contains("Type:     validation"));
    let issue_id = issue_id_by_title(dir.path(), "Typed issue");

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "show failed: {stderr}");
    assert!(stdout.contains("Typed issue"));
    assert!(stdout.contains("Category: todo"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "list failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("validation"));

    let (success, _, stderr) = run_atelier(dir.path(), &["export"]);
    assert!(success, "export failed: {stderr}");
    let issue_record = std::fs::read_to_string(
        dir.path()
            .join(".atelier/issues")
            .join(format!("{issue_id}.md")),
    )
    .unwrap();
    assert!(issue_record.contains("issue_type: \"validation\"\n"));
    assert!(issue_record.contains("labels:\n- \"epic\"\n"));
}

#[test]
fn test_import_beads_reports_mapping_without_tracker_provenance() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/beads/issues.manual.jsonl");

    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["import-beads", fixture.to_str().unwrap()]);
    assert!(success, "import-beads failed: {stderr}");
    assert!(stdout.contains("imported issues: 3"));
    assert!(stdout.contains("blocking relationships: 1"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "show", "3"]);
    assert!(success, "mapped show failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("[task]"));
    assert!(stdout.contains("Parent: atelier-0001"));
    assert!(stdout.contains("atelier-0002"));
    assert!(!stdout.contains("beads:"));

    let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "blocked", "3"]);
    assert!(success, "mapped issue blocked failed: {stderr}");
    assert!(stdout.contains("atelier-0003"));
    assert!(stdout.contains("atelier-0002"));
}

// ============================================================
// Unicode E2E Tests - Comprehensive multi-byte character handling
// ============================================================

/// Test issue creation and listing with Unicode arrows
#[test]
fn test_unicode_arrows_in_title() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // The exact issue that caused the original panic
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Add keyboard shortcuts for swiping (← →)",
        ],
    );
    assert!(success);

    // List should not panic
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("←") || stdout.contains("...")); // Either shows or truncates
}

/// Test various Unicode characters in issue titles
#[test]
fn test_unicode_variety_in_titles() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode_titles = vec![
        "日本語タイトル",                 // Japanese
        "中文标题测试",                   // Chinese
        "Тест на русском языке",          // Russian
        "العربية اختبار",                 // Arabic (RTL)
        "🎉 Emoji celebration 🎊🎈",      // Emoji
        "Mixed: Hello 世界 مرحبا мир 🌍", // Mixed scripts
        "Math: ∑∏∫∂ √∞ ≈≠≤≥",             // Math symbols
        "Arrows: ← → ↑ ↓ ↔ ↕ ⇐ ⇒",        // Arrows
        "Currency: $ € £ ¥ ₹ ₽ ₿",        // Currency
        "Box: ─│┌┐└┘├┤┬┴┼",               // Box drawing
    ];

    for (i, title) in unicode_titles.iter().enumerate() {
        let (success, _, _) = run_atelier(dir.path(), &["issue", "create", title]);
        assert!(success, "Failed to create issue with title: {}", title);

        // Verify it can be shown without panic
        let id = (i + 1).to_string();
        let (success, _, _) = run_atelier(dir.path(), &["issue", "show", &id]);
        assert!(
            success,
            "Failed to show issue #{} with title: {}",
            i + 1,
            title
        );
    }

    // List all - tests truncation on long Unicode
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test Unicode in descriptions and comments
#[test]
fn test_unicode_in_descriptions_and_comments() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create with Unicode description
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Unicode test",
            "-d",
            "Description with 日本語 and émojis 🚀",
        ],
    );
    assert!(success);

    // Add Unicode comment
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "comment", "1", "Comment: ← back, → forward, ↑ up"],
    );
    assert!(success);

    // Show should display without panic
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success);
    assert!(
        stdout.contains("日本語"),
        "Show output should contain the Unicode description text, got: {}",
        stdout
    );
}

/// Test Unicode survives current list and show surfaces
#[test]
fn test_unicode_list_and_show() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "日本語のテスト"]);
    run_atelier(dir.path(), &["issue", "create", "Test with arrows ← →"]);
    run_atelier(dir.path(), &["issue", "create", "Emoji test 🎉"]);

    let (success, list_out, stderr) =
        run_atelier(dir.path(), &["issue", "list", "--status", "all"]);
    assert!(success, "issue list failed: {stderr}");
    assert!(list_out.contains("日本語のテスト"), "{list_out}");
    assert!(list_out.contains("Test with arrows ← →"), "{list_out}");
    assert!(list_out.contains("Emoji test 🎉"), "{list_out}");

    let (success, show_out, stderr) = run_atelier(dir.path(), &["issue", "show", "1"]);
    assert!(success, "issue show failed: {stderr}");
    assert!(show_out.contains("日本語のテスト"), "{show_out}");
}

/// Test very long Unicode strings (stress test truncation)
#[test]
fn test_unicode_long_string_truncation() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Create title that's definitely longer than truncation limit
    // Using 3-byte UTF-8 chars (←) to maximize byte/char mismatch
    let long_arrows = "←".repeat(60);
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", &format!("Long: {}", long_arrows)],
    );
    assert!(success);

    // List must not panic on truncation
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
    assert!(stdout.contains("...") || stdout.contains("Long:"));

    // Create title with mixed byte-length chars
    let mixed = "a←b→c↑d↓e🎉f".repeat(10);
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", &mixed]);
    assert!(success);

    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}

/// Test blocked/ready lists with Unicode
#[test]
fn test_unicode_in_dependencies() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "ブロッカー (blocker) ←"]);
    run_atelier(dir.path(), &["issue", "create", "待機中 (waiting) →"]);
    run_atelier(dir.path(), &["issue", "block", "2", "1"]);

    // Blocked list with Unicode
    let (success, _, _) = run_atelier(dir.path(), &["issue", "blocked"]);
    assert!(success);

    // Ready list
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list", "--ready"]);
    assert!(success);
}

/// Test export/import preserves Unicode
#[test]
#[ignore = "reason: obsolete legacy command surface removed; owner: cli; issue: atelier-jqds; product: no; blocking: no"]
fn test_unicode_export_import_roundtrip() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    let unicode_title = "Test: 日本語 ← → 🎉";
    let unicode_desc = "Description: 中文 العربية Русский";

    run_atelier(
        dir.path(),
        &["issue", "create", unicode_title, "-d", unicode_desc],
    );
    run_atelier(dir.path(), &["issue", "comment", "1", "コメント (comment)"]);

    // Export
    let export_path = dir.path().join("unicode_backup.json");
    let (success, _, _) = run_atelier(
        dir.path(),
        &["export", "-o", export_path.to_str().unwrap(), "-f", "json"],
    );
    assert!(success);

    // Import to new location
    let dir2 = tempdir().unwrap();
    init_atelier(dir2.path());
    std::fs::copy(&export_path, dir2.path().join("unicode_backup.json")).unwrap();

    let (success, _, _) = run_atelier(
        dir2.path(),
        &[
            "import",
            dir2.path().join("unicode_backup.json").to_str().unwrap(),
        ],
    );
    assert!(success);

    // Verify Unicode preserved
    let unicode_id = issue_id_by_title(dir2.path(), unicode_title);
    let (success, stdout, _) = run_atelier(dir2.path(), &["issue", "show", &unicode_id]);
    assert!(success);
    assert!(
        stdout.contains("日本語") || stdout.contains("Test:"),
        "Unicode should be preserved in export/import"
    );
}

/// Test zero-width and special Unicode characters
#[test]
fn test_unicode_special_characters() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    // Zero-width characters (shouldn't break anything)
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "create",
            "Test\u{200B}with\u{200B}zero\u{200B}width",
        ],
    );
    assert!(success);

    // RTL override characters
    let (success, _, _) = run_atelier(
        dir.path(),
        &["issue", "create", "Test \u{202E}desrever\u{202C} normal"],
    );
    assert!(success);

    // Combining characters (accent marks)
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Café résumé naïve"]);
    assert!(success);

    // All should list without panic
    let (success, _, _) = run_atelier(dir.path(), &["issue", "list"]);
    assert!(success);
}
