//! Runs `git` and the built `atelier` binary in a fixture repository, shared
//! by the scenario tests that drive whole workflows through the CLI.
use std::{path::Path, process::Command};

/// Runs git and returns its trimmed stdout, failing the test on a non-zero exit.
pub fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

/// Runs atelier and returns whether it succeeded with its stdout and stderr.
pub fn cli(dir: &Path, args: &[&str]) -> (bool, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // `land` reports work in progress on a work branch and exits
    // successfully (atelier-md0v); tests read that report, whose verdict is
    // `land: work in progress`, as not passed, as they read a refusal.
    let in_progress = stdout.lines().last().is_some_and(|line| {
        line.starts_with("land: work in progress") || line.starts_with("work in progress")
    });
    (
        output.status.success() && !in_progress,
        format!("{stdout}{}", String::from_utf8_lossy(&output.stderr)),
    )
}

/// Runs atelier, failing the test unless it succeeds, and returns its trimmed output.
pub fn ok(dir: &Path, args: &[&str]) -> String {
    let (passed, output) = cli(dir, args);
    assert!(passed, "atelier {args:?}: {output}");
    output.trim().into()
}

/// Commits everything in the working tree.
pub fn commit(dir: &Path, message: &str) {
    git(dir, &["add", "."]);
    git(dir, &["commit", "-qm", message]);
}
