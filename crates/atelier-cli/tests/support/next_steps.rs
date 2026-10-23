//! Runs every fix a gate prints, verbatim, so a printed `Next:` command is
//! proven to work, not only to read well (atelier-h1nc reviews).
use std::{path::Path, process::Command};

/// The reason substituted for `"…"`: a Close Reason with every report part,
/// so a close the policy requires a report for accepts it.
const REASON: &str = "Fixed by the gate's printed command.\n\n### Deviations\nNone.\n\n### Decisions\nNone.\n\n### Surprises\nNone.\n\n### Open items\nNone.";

/// The alternatives each `Next:` in `output` offers, each as the commands to
/// run in order. A fix that is one bare command is one step; a sentence
/// quotes each command in backticks, `, or ` separates alternatives, and
/// `, then ` steps within one.
pub fn fixes(output: &str) -> Vec<Vec<String>> {
    output
        .lines()
        .filter_map(|line| line.split_once(". Next: ").map(|(_, fix)| fix))
        .flat_map(|fix| {
            if !fix.contains('`') {
                return vec![vec![fix.to_owned()]];
            }
            fix.split(", or ")
                .map(|alternative| {
                    alternative
                        .split('`')
                        .skip(1)
                        .step_by(2)
                        .map(str::to_owned)
                        .collect::<Vec<_>>()
                })
                .filter(|steps| !steps.is_empty())
                .collect()
        })
        .collect()
}

/// Runs each alternative every `Next:` in `output` offers in its own copy of
/// `repo`, with the placeholders a person fills in substituted, and asserts
/// every command exits 0. Returns how many alternatives ran.
pub fn assert_fixes_run(repo: &Path, output: &str) -> usize {
    let bin = Path::new(env!("CARGO_BIN_EXE_atelier")).parent().unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let alternatives = fixes(output);
    assert!(!alternatives.is_empty(), "no Next: fix in {output}");
    for steps in &alternatives {
        let copy = tempfile::tempdir().unwrap();
        let status = Command::new("cp")
            .args(["-a", &format!("{}/.", repo.display())])
            .arg(copy.path())
            .status()
            .unwrap();
        assert!(status.success());
        for step in steps {
            let git = |args: &[&str]| {
                String::from_utf8(
                    Command::new("git")
                        .current_dir(copy.path())
                        .args(args)
                        .output()
                        .unwrap()
                        .stdout,
                )
                .unwrap()
            };
            let state = || {
                [
                    git(&["rev-parse", "HEAD"]),
                    git(&["branch", "--show-current"]),
                    git(&["status", "--porcelain", "--untracked-files=all"]),
                ]
            };
            let before = state();
            let head = before[0].clone();
            let command = step
                .replace("\"…\"", "\"$FIX_REASON\"")
                .replace("<identity>", "Reviewer")
                .replace("<reviewer>", "Reviewer")
                .replace("<commit>", head.trim());
            assert!(
                !command.contains('<'),
                "unfilled placeholder in {step:?} of {output}"
            );
            let run = Command::new("sh")
                .current_dir(copy.path())
                .env("PATH", &path)
                .env("FIX_REASON", REASON)
                .args(["-c", &command])
                .output()
                .unwrap();
            assert!(
                run.status.success(),
                "fix {command:?} (from {steps:?}) failed:\n{}{}\nin output:\n{output}",
                String::from_utf8_lossy(&run.stdout),
                String::from_utf8_lossy(&run.stderr)
            );
            // A fix that only reads, such as the reviewer brief or the
            // transition listing, changes nothing; any other must change the
            // records or the checkout, so a no-op that exits 0 still fails.
            let words = command.split_whitespace().collect::<Vec<_>>();
            let reads = command.starts_with("atelier review request ")
                || command.starts_with("atelier issue show ")
                || words.len() == 4 && command.starts_with("atelier issue transition ");
            assert!(
                reads || state() != before,
                "fix {command:?} exited 0 but changed nothing:\n{}{}\nin output:\n{output}",
                String::from_utf8_lossy(&run.stdout),
                String::from_utf8_lossy(&run.stderr)
            );
        }
    }
    alternatives.len()
}
