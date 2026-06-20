use std::process::Command;
use std::thread;

use super::harness::{assert_stdout_contains, SmokeHarness};

// ============================================================================
// Boundary Attacks
// ============================================================================

#[test]
fn test_boundary_title_exact_512() {
    let h = SmokeHarness::new();
    let title = "a".repeat(512);
    let result = h.run_ok(&["issue", "create", &title]);
    assert!(result.stdout.contains("Created issue"));

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert!(show.stdout.contains(&title));
}

#[test]
fn test_boundary_title_over_513() {
    let h = SmokeHarness::new();
    let title = "a".repeat(513);
    let result = h.run(&["issue", "create", &title]);
    if result.success {
        let issue_id = h.issue_id(1);
        let show = h.run_ok(&["issue", "show", &issue_id]);
        assert!(show.stdout.contains(&title[..50]));
    } else {
        assert!(
            result.stderr.contains("exceeds") || result.stderr.contains("maximum length"),
            "Expected error about length, got stderr: {}",
            result.stderr
        );
    }
}

#[test]
fn test_boundary_title_null_bytes() {
    let h = SmokeHarness::new();
    let output = Command::new(&h.atelier_bin)
        .current_dir(h.temp_dir.path())
        .args(["create", "test\x00null"])
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let list = h.run_ok(&["issue", "list", "-s", "all"]);
                assert!(list.success);
            }
        }
        Err(e) => {
            assert!(
                e.kind() == std::io::ErrorKind::InvalidInput,
                "Expected InvalidInput error for null byte, got: {:?}",
                e.kind()
            );
        }
    }
}

#[test]
fn test_boundary_label_exact_128() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Label boundary test"]);
    let issue_id = h.issue_id(1);

    let label = "a".repeat(128);
    h.run_ok(&["issue", "update", &issue_id, "--label", &label]);

    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert!(show.stdout.contains(&label));
}

#[test]
fn test_boundary_label_over_129() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Label boundary test"]);
    let issue_id = h.issue_id(1);

    let label = "a".repeat(129);
    let result = h.run(&["issue", "update", &issue_id, "--label", &label]);
    if result.success {
        let show = h.run_ok(&["issue", "show", &issue_id]);
        assert!(show.stdout.contains(&label[..50]));
    } else {
        assert!(
            result.stderr.contains("exceeds") || result.stderr.contains("maximum length"),
            "Expected error about label length, got stderr: {}",
            result.stderr
        );
    }
}

// These tests are skipped on Windows because the OS command line length limit
// (~32KB on Windows via CreateProcessW) prevents passing 64KB+ as CLI arguments.
#[cfg(not(target_os = "windows"))]
#[test]
fn test_removed_description_flag_rejects_large_value() {
    let h = SmokeHarness::new();
    let desc = "b".repeat(65_536);
    let result = h.run(&["issue", "create", "Desc boundary test", "-d", &desc]);
    assert!(!result.success, "issue create -d should be removed");
    assert!(
        result.stderr.contains("unexpected argument") || result.stderr.contains("Usage:"),
        "Expected removed description flag to be rejected, got stderr: {}",
        result.stderr
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_removed_description_flag_rejects_oversized_value() {
    let h = SmokeHarness::new();
    let desc = "b".repeat(65_537);
    let result = h.run(&["issue", "create", "Desc boundary test", "-d", &desc]);
    assert!(!result.success, "issue create -d should be removed");
    assert!(
        result.stderr.contains("unexpected argument") || result.stderr.contains("Usage:"),
        "Expected removed description flag to be rejected, got stderr: {}",
        result.stderr
    );
}

#[test]
fn test_boundary_empty_title() {
    let h = SmokeHarness::new();
    let _result = h.run(&["issue", "create", ""]);
    let list = h.run_ok(&["issue", "list", "-s", "all"]);
    assert!(list.success);
}

#[test]
fn test_boundary_whitespace_title() {
    let h = SmokeHarness::new();
    let _result = h.run(&["issue", "create", "   "]);
    let list = h.run_ok(&["issue", "list", "-s", "all"]);
    assert!(list.success);
}

#[test]
fn test_boundary_priority_invalid() {
    let h = SmokeHarness::new();
    let result = h.run_err(&["issue", "create", "Priority test", "-p", "hgih"]);
    assert!(
        result.stderr.contains("Invalid priority")
            || result.stderr.contains("invalid")
            || result.stderr.contains("hgih"),
        "Expected error about invalid priority, got stderr: {}",
        result.stderr
    );
}

#[test]
fn test_boundary_priority_case() {
    let h = SmokeHarness::new();
    let result = h.run_err(&["issue", "create", "Priority case test", "-p", "High"]);
    assert!(
        result.stderr.contains("Invalid priority")
            || result.stderr.contains("invalid")
            || result.stderr.contains("High"),
        "Expected error about invalid priority, got stderr: {}",
        result.stderr
    );
}

#[test]
fn test_boundary_status_invalid() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Test issue"]);

    let result = h.run(&["issue", "list", "-s", "bogus"]);
    if result.success {
        assert!(
            !result.stdout.contains("Test issue"),
            "Invalid status should not match real issues"
        );
    } else {
        assert!(
            result.stderr.contains("Invalid status")
                || result.stderr.contains("invalid")
                || result.stderr.contains("bogus"),
            "Expected error about invalid status, got stderr: {}",
            result.stderr
        );
    }
}

// ============================================================================
// SQL Injection
// ============================================================================

#[test]
fn test_inject_sql_title() {
    let h = SmokeHarness::new();
    let payload = "'; DROP TABLE issues; --";
    h.run_ok(&["issue", "create", payload]);

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, payload);

    h.run_ok(&["issue", "create", "Normal issue after injection"]);

    let list = h.run_ok(&["issue", "list", "-s", "all"]);
    // Both issues should exist
    assert!(list.stdout.contains("Normal issue after injection"));
    assert!(list.stdout.contains(payload) || list.stdout.contains("DROP TABLE"));
}

#[test]
fn test_inject_sql_search() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Findable issue"]);
    h.run_ok(&["issue", "create", "Another issue"]);

    let _result = h.run_ok(&["search", "% OR 1=1 --"]);
    // DB should remain intact
    let list = h.run_ok(&["issue", "list", "-s", "all"]);
    assert!(list.stdout.contains("Findable issue"));
    assert!(list.stdout.contains("Another issue"));
}

#[test]
fn test_inject_sql_label() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Label injection test"]);
    let issue_id = h.issue_id(1);

    let payload = "'; DELETE FROM labels; --";
    h.run_ok(&["issue", "update", &issue_id, "--label", payload]);

    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, payload);

    h.run_ok(&["issue", "update", &issue_id, "--label", "safe-label"]);
    let show2 = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show2, "safe-label");
}

#[test]
fn test_inject_sql_comment() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Comment injection test"]);
    let issue_id = h.issue_id(1);

    let payload = "comment'); DELETE FROM comments; --";
    h.run_ok(&["issue", "note", &issue_id, payload]);

    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, payload);
}

// ============================================================================
// Shell Metacharacters
// ============================================================================

#[test]
fn test_inject_shell_title() {
    let h = SmokeHarness::new();
    let payload = "Issue with $(whoami) and `id` and $HOME";
    h.run_ok(&["issue", "create", payload]);

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, "$(whoami)");
    assert_stdout_contains(&show, "`id`");
    assert_stdout_contains(&show, "$HOME");
}

#[test]
fn test_inject_shell_comment() {
    let h = SmokeHarness::new();
    h.run_ok(&["issue", "create", "Shell comment test"]);
    let issue_id = h.issue_id(1);

    let payload = "Running $(rm -rf /) and `cat /etc/shadow` for $USER";
    h.run_ok(&["issue", "note", &issue_id, payload]);

    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, "$(rm -rf /)");
}

// ============================================================================
// Unicode Edge Cases
// ============================================================================

#[test]
fn test_unicode_emoji_title() {
    let h = SmokeHarness::new();
    let title =
        "Fix rendering of \u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466} emoji";
    h.run_ok(&["issue", "create", title]);

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(
        &show,
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}",
    );
}

#[test]
fn test_unicode_rtl_title() {
    let h = SmokeHarness::new();
    let title = "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{0628}\u{0627}\u{0644}\u{0639}\u{0627}\u{0644}\u{0645}";
    h.run_ok(&["issue", "create", title]);

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, title);
}

#[test]
fn test_unicode_mixed_scripts() {
    let h = SmokeHarness::new();
    let title = "Hello \u{041F}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442} \u{4F60}\u{597D} \u{0928}\u{092E}\u{0938}\u{094D}\u{0924}\u{0947}";
    h.run_ok(&["issue", "create", title]);

    let issue_id = h.issue_id(1);
    let show = h.run_ok(&["issue", "show", &issue_id]);
    assert_stdout_contains(&show, title);
}

// ============================================================================
// Corruption Recovery
// ============================================================================

#[cfg(unix)]
#[test]
fn test_corrupt_db_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let h = SmokeHarness::new();
    let perms = std::fs::Permissions::from_mode(0o444);
    std::fs::set_permissions(h.db_path(), perms).unwrap();

    if std::fs::OpenOptions::new()
        .write(true)
        .open(h.db_path())
        .is_ok()
    {
        let perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(h.db_path(), perms).unwrap();
        return;
    }

    let result = h.run(&["issue", "create", "Should fail"]);
    assert!(
        !result.success,
        "Expected failure when DB is read-only, but command succeeded.\nstdout: {}\nstderr: {}",
        result.stdout, result.stderr
    );

    let perms = std::fs::Permissions::from_mode(0o644);
    std::fs::set_permissions(h.db_path(), perms).unwrap();
}

#[test]
fn test_corrupt_missing_db() {
    let h = SmokeHarness::new();
    std::fs::remove_file(h.db_path()).unwrap();

    let result = h.run(&["issue", "list"]);
    // Whether it succeeds (recreates DB) or fails (reports missing DB),
    // it should not panic.
    if result.success {
        let list = h.run_ok(&["issue", "list", "-s", "all"]);
        assert!(list.success);
    }
}

// ============================================================================
// Concurrency (inline — small test)
// ============================================================================

#[test]
fn test_concurrent_creates_5() {
    let h = SmokeHarness::new();
    let bin = h.atelier_bin.clone();
    let dir = h.temp_dir.path().to_path_buf();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let bin = bin.clone();
            let dir = dir.clone();
            thread::spawn(move || {
                let output = Command::new(&bin)
                    .current_dir(&dir)
                    .args(["issue", "create", &format!("Concurrent issue {}", i)])
                    .output()
                    .expect("failed to execute atelier");
                output.status.success()
            })
        })
        .collect();

    let mut successes = 0u32;
    for handle in handles {
        if handle.join().expect("thread panicked") {
            successes += 1;
        }
    }

    assert!(
        successes >= 1,
        "At least one concurrent create should succeed, got 0",
    );

    let result = h.run_ok(&["issue", "list", "-s", "all"]);
    assert!(result.success);
}
