---
created_at: "2026-11-04T23:08:01.784702835+00:00"
id: "atelier-cej5"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-cej5
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:34:25.081529066+00:00"
status: "done"
title: "Keep path_log's date-order parity test inside nextest's timeout on macOS"
updated_at: "2026-11-04T23:34:25.081529066+00:00"
---

## Goal

Stop `atelier-app` `path_log::tests::matches_git_when_dates_run_backwards_or_tie` from timing out on macOS CI (nextest default `terminate-after` 6 × 10s = 60s). It failed master runs 41144966277 and 40725124384.

## Done when

The two date steps run as separate tests, each doing the work of one `assert_parity` over `branchy`, so each sits near its siblings' ~23s on macOS; the nextest timeout is unchanged; and the macOS CI job for the PR passes with both tests reported.

## Context

Diagnosis (CI logs of both failed runs, plus passing macOS runs): the test is not hanging. It runs the `branchy` history and `assert_parity` twice (commit-date steps -60 and 0); `assert_parity` spawns one `git log` per start commit × path spec × listing × reverse, a few thousand processes per call. Passing macOS runs take 48.6s and 51.0s for this test, against 22-24s for `matches_git_over_merges_octopus_and_criss_cross`, which does the same work once. Locally (Linux) 7.6s vs 3.8s. Under the parallel load of the full suite the doubled workload crosses 60s.

## Close Reason

The date-order parity test is now two tests, `matches_git_when_dates_run_backwards` and `matches_git_when_dates_tie`, each one `assert_parity` over `branchy`; on PR #18's macOS CI they took 19-24s each, in line with their siblings, with nextest's timeout unchanged.

### Deviations
None.

### Decisions
Split the work rather than raise the timeout or reduce the parity coverage: the cause was a doubled, deterministic workload of `git log` processes, not a hang.

### Surprises
The test was never close to flaky in the usual sense: passing macOS runs already took 48-51s of the 60s budget.

### Open items
None. Each parity test still spawns thousands of git processes; if macOS runners slow further the siblings could approach the limit too.
