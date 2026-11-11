---
schema: atelier.item
schema_version: 3
id: atelier-lkuy
title: Rewrite the Mission Overview entry in plain terms
status: draft
mission: atelier-4pzj
labels:
- cli-output
- work-views
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.986164681Z
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

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-om6l (Show what holds a mission up: a Waiting on section, what Needs a Human items hold, and the overview's Waiting on line)
