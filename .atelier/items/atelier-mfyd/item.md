---
schema: atelier.item
schema_version: 3
id: atelier-mfyd
title: Count ready work net of items in flight on other branches
status: dropped
mission: atelier-4pzj
labels:
- orientation
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T04:01:55.112626666Z
---

## Goal

After atelier-7n5u, ready rows in `status`, `work ready` and `work all` carry `↳ <status> on <branch>` marks, but the counts do not: `Ready work: 5` and the Next Action "5 ready issue(s) available" include items that are in progress or done on other local branches. In the validation of mission atelier-1w8c, 3 of 5 ready items were in flight elsewhere, so the headline count overstated what an agent can start.

## Done when

- [ ] `status` and `work ready` state how many ready items are in flight on other branches next to the ready count (for example `Ready work: 5 (3 in flight elsewhere)`), and the Next Action counts only items not in flight. Classification does not change. A session test covers it.

## Context

`crates/atelier-cli/src/commands/status.rs`, `git::branch_summaries` / `in_flight`. Found validating atelier-1w8c on 2026-10-17.

## Report

Dropped: Describes the old ready count. In 0.5 `status` lists what other threads hold separately from what is ready to start.
