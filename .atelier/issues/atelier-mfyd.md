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
status: "backlog"
title: "Count ready work net of items in flight on other branches"
updated_at: "2026-10-17T04:01:55.177363540+00:00"
---

## Goal

After atelier-7n5u, ready rows in `status`, `work ready` and `work all` carry `↳ <status> on <branch>` marks, but the counts do not: `Ready work: 5` and the Next Action "5 ready issue(s) available" include items that are in progress or done on other local branches. In the validation of mission atelier-1w8c, 3 of 5 ready items were in flight elsewhere, so the headline count overstated what an agent can start.

## Done when

`status` and `work ready` state how many ready items are in flight on other branches next to the ready count (for example `Ready work: 5 (3 in flight elsewhere)`), and the Next Action counts only items not in flight. Classification does not change. A session test covers it.

## Context

`crates/atelier-cli/src/commands/status.rs`, `git::branch_summaries` / `in_flight`. Found validating atelier-1w8c on 2026-10-17.
