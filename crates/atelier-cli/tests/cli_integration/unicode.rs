use super::*;

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
        let id = issue_ref(dir.path(), i + 1);
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

    // Create with Unicode description through canonical Markdown.
    let (success, _, _) = run_atelier(dir.path(), &["issue", "create", "Unicode test"]);
    assert!(success);
    let issue_id = issue_ref(dir.path(), 1);
    set_issue_description(
        dir.path(),
        &issue_id,
        "Description with 日本語 and émojis 🚀",
    );
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(success, "rebuild after description edit failed: {stderr}");

    // Add Unicode comment
    let (success, _, _) = run_atelier(
        dir.path(),
        &[
            "issue",
            "note",
            &issue_id,
            "Comment: ← back, → forward, ↑ up",
        ],
    );
    assert!(success);

    // Show should display without panic
    let (success, stdout, _) = run_atelier(dir.path(), &["issue", "show", &issue_id]);
    assert!(success);
    assert!(
        stdout.contains("日本語"),
        "Show output should contain the Unicode description text, got: {}",
        stdout
    );
}

/// Test search with Unicode queries
#[test]
fn test_unicode_search() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());

    run_atelier(dir.path(), &["issue", "create", "日本語のテスト"]);
    run_atelier(dir.path(), &["issue", "create", "Test with arrows ← →"]);
    run_atelier(dir.path(), &["issue", "create", "Emoji test 🎉"]);

    // Search for Japanese
    let (success, _, _) = run_atelier(dir.path(), &["search", "日本"]);
    assert!(success);

    // Search for emoji
    let (success, _, _) = run_atelier(dir.path(), &["search", "🎉"]);
    assert!(success);

    // Search for arrow
    let (success, _, _) = run_atelier(dir.path(), &["search", "←"]);
    assert!(success);
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
