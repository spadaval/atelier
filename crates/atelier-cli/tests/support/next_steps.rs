//! Runs every fix a gate prints, verbatim, so a printed `Next:` command is
//! proven to work, not only to read well (atelier-h1nc reviews), then runs
//! the gate again to prove the failure it fixes is gone (atelier-r3oy).
use std::{path::Path, process::Command};

/// The reason substituted for `"…"`: a Close Reason with every report part,
/// so a close the policy requires a report for accepts it.
const REASON: &str = "Fixed by the gate's printed command.\n\n### Deviations\nNone.\n\n### Decisions\nNone.\n\n### Surprises\nNone.\n\n### Open items\nNone.";

/// A command a fix names, as opposed to an instruction a person carries
/// out, such as editing a file.
fn is_command(text: &str) -> bool {
    ["atelier ", "git ", "perl "]
        .iter()
        .any(|program| text.starts_with(program))
}

/// A fix that only reads, such as the reviewer brief or a listing: it
/// changes nothing, so the failure it points to stays until someone acts.
fn reads(command: &str) -> bool {
    let words = command.split_whitespace().collect::<Vec<_>>();
    command.starts_with("atelier review request ")
        || command.starts_with("atelier review list ")
        || command.starts_with("atelier issue show ")
        || command.starts_with("atelier work ")
        || words.len() == 4 && command.starts_with("atelier issue transition ")
}

/// One fix as printed: the failure line it belongs to, up to its cause, and
/// the commands of one alternative, in order.
pub struct Fix {
    pub failure: String,
    pub steps: Vec<String>,
}

/// The alternatives each `Next:` in `output` offers, each as the commands to
/// run in order. A fix that is one bare command is one step; a sentence
/// quotes each command in backticks, `, or ` separates alternatives, and
/// `, then ` steps within one. An instruction with no command is left out.
pub fn fixes(output: &str) -> Vec<Fix> {
    output
        .lines()
        .filter_map(|line| line.split_once(". Next: "))
        .flat_map(|(failure, fix)| {
            // A folded check's name can change once the fix runs.
            let failure = failure
                .split(" (also ")
                .next()
                .unwrap_or(failure)
                .to_owned();
            let alternatives = if !fix.contains('`') {
                vec![vec![fix.to_owned()]]
            } else {
                fix.split(", or ")
                    .map(|alternative| {
                        alternative
                            .split('`')
                            .skip(1)
                            .step_by(2)
                            .map(str::to_owned)
                            .collect::<Vec<_>>()
                    })
                    .collect()
            };
            alternatives
                .into_iter()
                .filter(|steps| !steps.is_empty() && steps.iter().all(|step| is_command(step)))
                .map(move |steps| Fix {
                    failure: failure.clone(),
                    steps,
                })
        })
        .collect()
}

/// Runs each alternative every `Next:` in `output` offers in its own copy of
/// `repo`, with the placeholders a person fills in substituted, and asserts
/// every command exits 0. A fix that does more than read must change the
/// records or the checkout, and `atelier <gate>` run again must no longer
/// print the failure it fixes once the records it wrote are committed.
/// Returns how many alternatives ran.
pub fn assert_fixes_run(repo: &Path, output: &str, gate: &[&str]) -> usize {
    let bin = Path::new(env!("CARGO_BIN_EXE_atelier")).parent().unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let alternatives = fixes(output);
    assert!(!alternatives.is_empty(), "no Next: fix in {output}");
    for fix in &alternatives {
        let copy = tempfile::tempdir().unwrap();
        let status = Command::new("cp")
            .args(["-a", &format!("{}/.", repo.display())])
            .arg(copy.path())
            .status()
            .unwrap();
        assert!(status.success());
        let mut acted = false;
        for step in &fix.steps {
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
                    git(&["stash", "list"]),
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
                "fix {command:?} (from {:?}) failed:\n{}{}\nin output:\n{output}",
                fix.steps,
                String::from_utf8_lossy(&run.stdout),
                String::from_utf8_lossy(&run.stderr)
            );
            if reads(&command) {
                continue;
            }
            acted = true;
            assert!(
                state() != before,
                "fix {command:?} exited 0 but changed nothing:\n{}{}\nin output:\n{output}",
                String::from_utf8_lossy(&run.stdout),
                String::from_utf8_lossy(&run.stderr)
            );
        }
        if !acted {
            continue;
        }
        // Records a fix writes are committed with the work, as the gates
        // read them; anything else stays as the fix left it.
        let commit = |args: &[&str]| {
            Command::new("git")
                .current_dir(copy.path())
                .args(args)
                .output()
                .unwrap()
        };
        commit(&["add", ".atelier"]);
        if !commit(&["diff", "--cached", "--quiet"]).status.success() {
            assert!(commit(&["commit", "-qm", "records the fix wrote"])
                .status
                .success());
        }
        let rerun = Command::new(env!("CARGO_BIN_EXE_atelier"))
            .current_dir(copy.path())
            .args(gate)
            .output()
            .unwrap();
        let after = format!(
            "{}{}",
            String::from_utf8_lossy(&rerun.stdout),
            String::from_utf8_lossy(&rerun.stderr)
        );
        assert!(
            !after.contains(&format!("{}.", fix.failure))
                && !after.contains(&format!("{} (also ", fix.failure)),
            "after {:?}, `atelier {}` still reports {:?}:\n{after}",
            fix.steps,
            gate.join(" "),
            fix.failure
        );
    }
    alternatives.len()
}
