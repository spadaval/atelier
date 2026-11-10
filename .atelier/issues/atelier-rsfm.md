---
created_at: "2026-11-10T19:23:14.887937319+00:00"
id: "atelier-rsfm"
issue_type: "task"
labels:
- "deferred-findings"
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-rsfm
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3iyd"
  - kind: "issue"
    id: "atelier-ceko"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-xtvh"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Bound merge's CI check under the lock and fix its flaky wait test: follow-ups to atelier-xtvh"
updated_at: "2026-11-10T21:26:37.722392468+00:00"
---

## Goal

Collect the follow-ups deferred from review findings on atelier-xtvh, one per finding, filed by `atelier review defer atelier-xtvh`, and the defect found in atelier-xtvh after it merged (atelier-ceko). They are fixed together on one branch, with one review and one merge.

## Done when

- [x] Each child's Done when lines hold.
- [x] The Merge and forge sections of `docs/strategies/redesign-architecture.md` still match the code.

## Report

Both children's Done when lines hold, and the Merge section of `docs/strategies/redesign-architecture.md` says what the code now does (steps 2 and 5, the paragraph on constants, and the Testing line); the forge section needed no change. Two new tests in `next/crates/atelier-cli/tests/merge.rs` each failed on the code before the fix for its reason and pass after it: `a_poll_the_bound_ends_does_not_hide_what_an_earlier_poll_said` (atelier-ceko) and `a_look_at_ci_under_the_lock_that_stalls_is_ended_within_the_bound` (atelier-3iyd). All 154 tests of `next/` pass (`cargo nextest run --locked`), clippy has no warnings with `-D warnings`, and `cargo fmt -- --check` is clean.

The cause of the CI failure on pull request 106 is the one atelier-ceko states. The wait starts a poll whenever more than one poll interval is left, so the last poll can start as little as 50 ms before a test's bound. When that poll is still running at the bound, `gh` is killed, and its "did not answer in time" replaced what every earlier poll had said. Slowing the `gh` double's runs lookup by 0.1 s reproduced the CI output word for word, failing 4 of 6 runs of the test; the deterministic reproduction is the double's new `once` flag, which answers one poll and stalls every later one.

What `merge` prints at the bound now: what the last poll that answered said ("it is in progress (<link>)", "the latest was cancelled (<link>)", "no run of it has started"), even when a later poll failed or was cut off; and only when no poll answered, "the forge did not answer, as <why the first poll failed>". Under the lock, a question that has not answered after a minute is ended, and `merge` exits 3 with "the forge did not answer, as gh failed: gh api …/runs did not answer in time", releasing the lock.

### Decisions

- The limit under the lock is a constant, `LOCKED` (60 seconds), applied as `LOCKED.min(wait)`, so a test that shortens the wait through `ATELIER_TEST_CI_WAIT` shortens it too, and the hook keeps its two fields.
- When no poll answered, the first failure is reported rather than the last, so a poll the bound cuts off does not hide why the forge failed (`a_poll_that_fails_counts_as_not_finished` needs this).
- `Ci::Pending` now holds a `Result<Option<CiRun>, ForgeError>`, since its run and its error were never both set.

### Other tests of the wait

With the double's runs lookup slowed by 0.07, 0.2 and 0.3 s, three more tests failed on the code before the fix with the same "did not answer in time": `a_cancelled_run_is_not_a_finished_one`, `a_poll_that_fails_counts_as_not_finished` and `a_process_gh_leaves_holding_its_output_does_not_hold_up_the_bound`. The fix covers them, and they pass at every delay, except `a_cancelled_run_is_not_a_finished_one` at 0.3 s, whose 300 ms bound left no poll answered. That test, and `a_run_on_a_commit_before_a_change_outside_the_items_directory_does_not_count`, which also checks what the first poll said within 300 ms, now use a 2-second bound. The tests that wait through `Forge::waiting` (20-second bound), and the two that keep 300 ms but check only the start of the output, do not depend on timing this way.

### Open items

- Any test of the bound still needs one of its polls to answer before the bound (1 or 2 seconds of a shell script printing a file); a test of what `merge` says at the bound cannot avoid that. atelier-ceko's last line is ticked on that reading: the test that failed no longer depends on the poll in flight at the bound finishing.
- A stalled question under the lock prints "CI has not finished on …: the forge did not answer", although the wait had just seen CI pass. The reason it gives is accurate; the lead-in is the existing "not yet" text.
