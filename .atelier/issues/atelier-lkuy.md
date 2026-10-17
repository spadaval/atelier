---
created_at: "2026-10-17T01:10:57.986164681+00:00"
id: "atelier-lkuy"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Rewrite the Mission Overview entry in plain terms with useful blockers"
updated_at: "2026-10-17T01:10:57.986164681+00:00"
---

## Goal

`work missions` prints, per mission:
```
atelier-1w8c  draft  high  Orient a session from one view: ADR 0022 follow-ups
  Scope: 7 issues: 0 active, 2 ready, 3 blocked, 0 done, 2 backlog
  Open blockers: 2
  Drill down: atelier work mission atelier-1w8c
```
The human's verdict: "Scope" is a weird term, "Open blockers" is duplicative, and "Drill down" should become "More info: atelier work mission …". Also: zero buckets are padding; "Open blockers" counts distinct blocker IDs over the mission and every in-scope item including backlog (crates/atelier-app/src/mission_overview.rs:65-72), so r6bt shows `0 blocked` yet `Open blockers: 1`, and fixture j76w shows 8 mixing 3 in-scope items with 5 outside; `Scope: 1 issues`; draft and todo missions sort together (`category_rank` treats both as todo), so draft 1w8c lists above j76w, which has active work; a linked strategy is not shown.

## Done when

- Each entry is at most four lines, e.g.:
  ```
  atelier-j76w  todo  high  Ship the widget pipeline
    10 items: 1 active, 1 ready, 1 needs a human, 6 blocked, 1 backlog · strategy docs/strategies/x.md
    Waiting on: atelier-spvd (needs a human, holds 4), atelier-ubti (no mission, holds 3)
    More info: atelier work mission atelier-j76w
  ```
- No "Scope", "Open blockers" or "Drill down" labels; zero buckets omitted; plurals correct.
- The Waiting on line names up to two items at the end of blocker chains with how many each holds, and is omitted when nothing waits.
- Missions with active work sort before todo, todo before draft.
- `docs/product/issue-inventory-and-mission-overview.md` describes the new entry. Rendering tests updated.

## Scope

`crates/atelier-cli/src/commands/work.rs` `missions()` (around 204-304) and `mission_overview.rs`.

## Context

Unassigned-work grouping is atelier-ihns; the Waiting on facts are shared with the w.waiting-on item. Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.
