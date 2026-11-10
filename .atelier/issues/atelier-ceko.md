---
created_at: "2026-11-10T21:23:32.308509103+00:00"
id: "atelier-ceko"
issue_type: "bug"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-xtvh"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Report the last answer when the bound cuts off `merge`'s final poll: its test failed in CI on pull request 106"
updated_at: "2026-11-10T21:26:40.115392373+00:00"
---

## Goal

Stop `merge` from reporting "the forge did not answer" when the forge had answered and only its last poll was cut off by the bound. The test `at_the_bound_merge_exits_not_yet_and_changes_nothing` in `next/crates/atelier-cli/tests/merge.rs` failed for that reason in CI on pull request 106 (run 42903555518, job `next`), on code that did not touch `merge`, and passed when the job was rerun. With a 2-second test bound, the runner had a poll in flight when the bound came: `merge` printed "CI has not finished on 595ae93, the branch's head: the forge did not answer, as gh failed: gh api ... did not answer in time" where the test expects "it is in progress (https://github.com/owner/repo/actions/runs/2)". atelier-xtvh introduced the wait and the test. A test that fails at random refuses merges at random, and the message itself misleads an agent whose CI is running normally.

## Done when

- [x] The Report states the cause, with a reproduction that fails every time on the code before the fix (for example a `gh` double slow enough that a poll is in flight at the bound) and passes after it.
- [x] When the bound ends a poll that is in flight, `merge` reports what the last poll that answered said, such as that the run is in progress, with its link. It says the forge did not answer only when no poll answered during the wait.
- [x] A test whose `gh` double answers one poll with a run in progress and then stalls sees `merge` exit "not yet" at the bound, name that run with its link, and leave the branch, the base, the checkout and the remote as they were. The test that failed in CI no longer depends on a poll finishing inside a fixed time.

## Scope

`merge`'s wait in `next/crates/atelier-cli/` and its tests. No flag, config option or dependency. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge. The item that built the wait is atelier-xtvh; its Report describes the hook that shortens the bound and the poll interval in tests.

## Report

Fixed on atelier-rsfm; the cause, the reproduction and what `merge` now prints are in atelier-rsfm's Report. The cause was as stated: a poll started up to 50 ms before the bound was cut off, and its "did not answer in time" replaced the earlier answer. `a_poll_the_bound_ends_does_not_hide_what_an_earlier_poll_said` (the double answers one poll with a run in progress and stalls the next) failed every time before the fix and passes after it, naming run 1 with its link and leaving the branch, base, checkout and remote unchanged. `at_the_bound_merge_exits_not_yet_and_changes_nothing` passes unchanged; it still needs one of its polls to answer within 2 seconds.
