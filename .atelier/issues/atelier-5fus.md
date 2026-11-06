---
created_at: "2026-11-06T21:26:18.219762489+00:00"
id: "atelier-5fus"
issue_type: "task"
labels: []
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
status: "todo"
title: "Measure the bookkeeping of one standard lifecycle"
updated_at: "2026-11-06T21:26:18.219762489+00:00"
---

## Goal

Give the mission its baseline and its floor: how many record-only commits and review files one standard lifecycle produces on master today, and how many of those commits are forced because a record pins a commit it cannot be part of.

## Done when

- A scenario test in the CLI test suite drives, in a fixture repository: start an issue, record evidence, close it, a review with three findings, dispose one fix_now with a resolution commit and re-approval, one defer with a follow-up issue, one no_action, then merge. It asserts the counts of record-only commits and of review files the lifecycle produced, against the current numbers.
- An evidence receipt on this issue records the baseline counts from running that test on master at the mission start commit.
- A note on this issue says, for each record-only commit in the scenario, whether it is forced by pinning (the record names a commit it cannot be in) or by the current commands committing records on their own.

## Context

The mission strategy is docs/strategies/lighter-workflow.md (criterion lifecycle-cost). Later increments change these counts; the test's expected numbers are updated by them, which shows each reduction.
