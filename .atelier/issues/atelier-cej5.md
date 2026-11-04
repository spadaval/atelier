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
status: "in_progress"
title: "Keep path_log's date-order parity test inside nextest's timeout on macOS"
updated_at: "2026-11-04T23:08:16.521942425+00:00"
---

## Goal

Stop `atelier-app` `path_log::tests::matches_git_when_dates_run_backwards_or_tie` from timing out on macOS CI (nextest default `terminate-after` 6 × 10s = 60s). It failed master runs 41144966277 and 40725124384.

## Done when

The two date steps run as separate tests, each doing the work of one `assert_parity` over `branchy`, so each sits near its siblings' ~23s on macOS; the nextest timeout is unchanged; and the macOS CI job for the PR passes with both tests reported.

## Context

Diagnosis (CI logs of both failed runs, plus passing macOS runs): the test is not hanging. It runs the `branchy` history and `assert_parity` twice (commit-date steps -60 and 0); `assert_parity` spawns one `git log` per start commit × path spec × listing × reverse, a few thousand processes per call. Passing macOS runs take 48.6s and 51.0s for this test, against 22-24s for `matches_git_over_merges_octopus_and_criss_cross`, which does the same work once. Locally (Linux) 7.6s vs 3.8s. Under the parallel load of the full suite the doubled workload crosses 60s.
