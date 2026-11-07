---
schema: atelier.review
schema_version: 2
id: atelier-kt8y
title: finding on atelier-2bjz @6ea3eb5f
target: atelier-2bjz
identity: claude-review-2bjz
commit: 6ea3eb5f5c3cd2e45f14349fa20da5d4d957a7d1
created_at: 2026-11-05T00:08:16.931979356Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: claude-orchestrator
    commit: c9d894f442e7020c50761d3fde8b85a22a2990da
    recorded_at: 2026-11-05T00:08:52.736439243Z
    reason: The timeout is the old combined path_log test; atelier-cej5 split it on master (9bc7576c), and this branch's sync with master brings the fix in before merge.
    migrated_from: atelier-l6x3
---

medium: CI run 41934286418 (PR #20) failed on macOS: atelier-app path_log::tests::matches_git_when_dates_run_backwards_or_tie hit nextest's 60s timeout under --features test-hooks; the same test took 54.6s on macOS without hooks and 6.7s locally on Linux. Linux CI (both suites) passed, and the other 955 macOS tests passed. This branch does not touch path_log.rs, so the cause is the test sitting just under the limit on macOS runners, not this change. Impact: the merge gate (scripts/merge-ci-passed.sh) is red until a green rerun. Suggested: rerun the macOS job before atelier merge; as a separate follow-up, shrink that test's fixture or give it a longer slow-timeout override, since it will keep failing intermittently on every PR.
