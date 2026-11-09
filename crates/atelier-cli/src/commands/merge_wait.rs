//! `merge --wait`: when CI is the one gate left, poll it until the run
//! finishes or the timeout passes (docs/product/reviews-and-merge.md,
//! "Waiting for CI with `--wait`"). A run that passes lets the merge go on;
//! anything else leaves `ci.passed` refused with the reason it ended on.
use anyhow::Result;
use atelier_app::{
    ci_passed::{self, CiState, Verdict},
    review_gate::GateNode,
    workflow_policy::{WorkflowPolicy, CI_PASSED},
    workflow_validation::MergeHeads,
};
use std::path::Path;
use std::time::{Duration, Instant};

/// How often `--wait` reads CI.
const POLL: Duration = Duration::from_secs(30);
/// How long a run that has not started counts as pending: GitHub starts
/// runs a few seconds after a push, and a new pull request can take longer.
const GRACE: Duration = Duration::from_secs(5 * 60);

/// What `ci.passed` reads, as the merge gates passed it.
pub(super) struct Ci<'a> {
    pub checkout: &'a Path,
    pub policy: &'a WorkflowPolicy,
    pub issue_id: &'a str,
    pub heads: MergeHeads<'a>,
}

/// Waits for CI when `ci.passed` is the only failed gate, then rewrites
/// that gate with what CI said last. Any other failed gate refuses without
/// waiting.
pub(super) fn wait_for_ci(
    nodes: &mut [GateNode],
    ci: Ci<'_>,
    timeout: Duration,
    quiet: bool,
) -> Result<()> {
    let mut failed = nodes.iter_mut().filter(|node| !node.passed);
    let (Some(node), None) = (failed.next(), failed.next()) else {
        return Ok(());
    };
    if node.check != CI_PASSED {
        return Ok(());
    }
    let say = |line: &str| {
        if quiet {
            eprintln!("{line}");
        } else {
            println!("{line}");
        }
    };
    let started = Instant::now();
    let (poll, grace) = (poll(), grace());
    let mut last: Option<String> = None;
    let verdict = loop {
        let verdict = ci_passed::evaluate(ci.checkout, ci.policy, ci.issue_id, Some(ci.heads))?;
        let waited = started.elapsed();
        // A run that has not started is pending for its grace period; a
        // branch with no pull request can only get one from `--push`.
        let pending = match verdict.state {
            CiState::Pending => true,
            CiState::NoRun => waited < grace,
            CiState::NoPullRequest => ci.heads.push && waited < grace,
            CiState::Passed | CiState::Failed | CiState::Unreachable => false,
        };
        // A line each time CI's state changes, from the first pending one.
        if (pending || last.is_some()) && last.as_deref() != Some(verdict.reason.as_str()) {
            say(&if last.is_none() {
                format!(
                    "Waiting up to {} for CI: {}",
                    duration(timeout),
                    verdict.reason
                )
            } else {
                format!("CI: {}", verdict.reason)
            });
            last = Some(verdict.reason.clone());
        }
        if !pending {
            break ended(verdict, waited, grace);
        }
        if waited >= timeout {
            break Verdict {
                reason: format!(
                    "{}; merge waited {} for it",
                    verdict.reason,
                    duration(waited)
                ),
                ..verdict
            };
        }
        std::thread::sleep(poll.min(timeout - waited));
    };
    node.passed = verdict.passed();
    node.reason = verdict.reason;
    node.fix = verdict.next;
    Ok(())
}

/// The verdict CI ended on, saying how long a run that never started was
/// given.
fn ended(verdict: Verdict, waited: Duration, grace: Duration) -> Verdict {
    match verdict.state {
        CiState::NoRun | CiState::NoPullRequest if waited >= grace => Verdict {
            reason: format!(
                "{}, and none started within {} of waiting",
                verdict.reason,
                duration(grace)
            ),
            ..verdict
        },
        _ => verdict,
    }
}

/// Whole minutes, or seconds under a minute.
fn duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let (count, unit) = if seconds >= 60 {
        (seconds / 60, "minute")
    } else {
        (seconds, "second")
    };
    format!("{count} {unit}{}", if count == 1 { "" } else { "s" })
}

/// Tests set the poll interval with `ATELIER_TEST_MERGE_CI_POLL_MS`.
fn poll() -> Duration {
    test_millis("ATELIER_TEST_MERGE_CI_POLL_MS").unwrap_or(POLL)
}

/// Tests set the grace period with `ATELIER_TEST_MERGE_CI_GRACE_MS`.
fn grace() -> Duration {
    test_millis("ATELIER_TEST_MERGE_CI_GRACE_MS").unwrap_or(GRACE)
}

#[cfg(feature = "test-hooks")]
fn test_millis(name: &str) -> Option<Duration> {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .map(Duration::from_millis)
}
#[cfg(not(feature = "test-hooks"))]
fn test_millis(_: &str) -> Option<Duration> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_read_in_whole_minutes_or_seconds() {
        assert_eq!(duration(Duration::from_secs(0)), "0 seconds");
        assert_eq!(duration(Duration::from_secs(1)), "1 second");
        assert_eq!(duration(Duration::from_secs(61)), "1 minute");
        assert_eq!(duration(Duration::from_secs(30 * 60)), "30 minutes");
    }
}
