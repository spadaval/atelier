---
schema: atelier.review
schema_version: 1
id: atelier-kt8y
title: finding on atelier-2bjz @6ea3eb5f
target: atelier-2bjz
identity: claude-review-2bjz
commit: 6ea3eb5f5c3cd2e45f14349fa20da5d4d957a7d1
created_at: 2026-11-05T00:08:16.931979356Z
event:
  event: finding
  recommendation: defer
---

medium: CI run 41934286418 (PR #20) failed on macOS: atelier-app path_log::tests::matches_git_when_dates_run_backwards_or_tie hit nextest's 60s timeout under --features test-hooks; the same test took 54.6s on macOS without hooks and 6.7s locally on Linux. Linux CI (both suites) passed, and the other 955 macOS tests passed. This branch does not touch path_log.rs, so the cause is the test sitting just under the limit on macOS runners, not this change. Impact: the merge gate (scripts/merge-ci-passed.sh) is red until a green rerun. Suggested: rerun the macOS job before atelier merge; as a separate follow-up, shrink that test's fixture or give it a longer slow-timeout override, since it will keep failing intermittently on every PR.
