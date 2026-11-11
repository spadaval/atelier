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
    id: "atelier-4pzj"
    type: "advances"
  - kind: "issue"
    id: "atelier-om6l"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:38.544785940+00:00"
status: "superseded"
title: "Rewrite the Mission Overview entry in plain terms"
updated_at: "2026-11-11T03:55:38.544785940+00:00"
---

## Goal

Sumeet's verdict on `work missions`: "Scope" is a weird term, "Open blockers" is duplicative, and "Drill down" should be "More info". "More info" and omitting zero buckets have landed. Still left (checked 2026-11-07): entries say `Scope: 13 issues: …`; `Open blockers: N` counts distinct blocker IDs over the mission and every in-scope item, backlog included (crates/atelier-app/src/mission_overview.rs), so it disagrees with the blocked count; draft and todo missions sort together (`category_rank` treats both as todo), so a draft lists above a todo mission with active work; a linked strategy is not shown.

## Done when

- [ ] Each entry is at most three lines, e.g.
  ```
  atelier-j76w  todo  high  Ship the widget pipeline
    10 items: 1 active, 1 ready, 1 needs a human, 6 blocked, 1 backlog · strategy docs/strategies/x.md
    More info: atelier work mission atelier-j76w
  ```
- [ ] No "Scope" or "Open blockers" label, and plurals are correct.
- [ ] Missions with active work sort before todo, and todo before draft.
- [ ] docs/product/issue-inventory-and-mission-overview.md describes the entry, and rendering tests are updated.

## Scope

crates/atelier-cli/src/commands/work.rs `missions()` and crates/atelier-app/src/mission_overview.rs. The Waiting on line moved to atelier-om6l in decision atelier-mzpn; with it an entry is at most four lines.

## Context

Carries the part of atelier-1w8c's S3 that its validation found unmet.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It rewrites the `work missions` entry, and the What is removed table removes the `work` views; `list` heads each mission with its status and lead.
