---
created_at: "2026-10-17T04:01:55.112626666+00:00"
id: "atelier-mfyd"
issue_type: "bug"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:38.864398611+00:00"
status: "superseded"
title: "Count ready work net of items in flight on other branches"
updated_at: "2026-11-11T03:55:38.864398611+00:00"
---

## Goal

After atelier-7n5u, ready rows in `status`, `work ready` and `work all` carry `↳ <status> on <branch>` marks, but the counts do not: `Ready work: 5` and the Next Action "5 ready issue(s) available" include items that are in progress or done on other local branches. In the validation of mission atelier-1w8c, 3 of 5 ready items were in flight elsewhere, so the headline count overstated what an agent can start.

## Done when

`status` and `work ready` state how many ready items are in flight on other branches next to the ready count (for example `Ready work: 5 (3 in flight elsewhere)`), and the Next Action counts only items not in flight. Classification does not change. A session test covers it.

## Context

`crates/atelier-cli/src/commands/status.rs`, `git::branch_summaries` / `in_flight`. Found validating atelier-1w8c on 2026-10-17.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new `status` lists only the ready items `start` would accept now and counts the others by what stops them (run in a scratch clone: `14 other ready items cannot start yet: 5 wait on a blocker that is not done, 11 are in a mission that is not ready`).
