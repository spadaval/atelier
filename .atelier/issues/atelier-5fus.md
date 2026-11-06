---
created_at: "2026-11-06T21:26:18.219762489+00:00"
id: "atelier-5fus"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-5fus
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Measure the bookkeeping of one standard lifecycle"
updated_at: "2026-11-06T23:42:36.494539297+00:00"
---

## Goal

Give the mission its baseline and its floor: how many record-only commits and review files one standard lifecycle produces with the product as it is at the mission start, and how many of those commits are forced rather than chosen.

## Done when

- A scenario test under crates/atelier-cli/tests/ drives the lifecycle-cost scenario exactly as the strategy defines it: a fixture repository with the `atelier init` starter policy; commands that accept `--commit-records` use it, and `atelier start` and `issue create` are each followed by one commit of exactly the records they wrote; start an issue, record one manual evidence receipt (`--result pass`), close it; one review (by an identity other than the author) that raises three findings, one of them `fix_now`, and does not approve; a fix commit; dispositions recorded by the author: `fix_now` with that resolution commit, `defer` with a follow-up issue, `no_action` with a reason (the last two accepted by the reviewer identity, as today's rules require); the reviewer reviews the fix and approves the head; merge.
- The test asserts two counts against today's numbers: record-only commits (commits on the work branch that change only files under `.atelier/`) and, for each finding, review files about it (the finding plus every review event that names it). Later items change these expected numbers, which shows each reduction.
- An evidence receipt on this issue, recorded at the branch head before any product change (product code equal to master at the commit the branch left from), records both counts as the baseline.
- A note on this issue classifies each record-only commit in the scenario as forced by pinning (the record names a commit it cannot be part of), forced by gate ordering (for example, the done claim before review, approval before merge), or chosen by how commands commit records today. The note says whether "at most half the baseline" is reachable given the forced commits; if it is not, it proposes a rewritten lifecycle-cost criterion for Sumeet.

## Scope

Test-only: a new test file, reusing the fixture helpers in crates/atelier-cli/tests/merge_gates.rs (moved to a shared test module if needed). No product code, policy or doc changes.

## Context

Strategy: docs/strategies/lighter-workflow.md (criterion lifecycle-cost). The review request brief tells reviewers to use `--commit-records` (crates/atelier-cli/src/commands/review_request.rs); `--stage-records` batching exists (docs/product/cli-surface.md) but the scenario must not switch to it, so reductions come from product changes.
