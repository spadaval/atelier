---
created_at: "2026-10-17T01:10:57.983995411+00:00"
id: "atelier-5qes"
issue_type: "bug"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-31l2"
    type: "supersedes"
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Order work-view rows with the shared blocker-first order"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

`atelier list` orders rows within each mission and label group by priority, with no regard to blockers: with atelier-1ci8 (medium) blocked by atelier-1a5k (low), links set for the test, `list --mission atelier-4pzj` prints atelier-1ci8 on line 14 and its blocker atelier-1a5k on line 43 (checked 2026-11-11 with the new binary in a converted scratch clone). A reader working down the list meets the blocked work before what unblocks it.

## Done when

- [ ] In `list --blocked`, and within each status group of `show <mission>`, a visible blocker comes before what it blocks, then rows sort by state, priority, update time and ID; `list` keeps its mission and label groups and orders each the same way.
- [ ] A test with chain a → b → c (c blocked by b, b by a) asserts that order in `list --blocked` and in `show <mission>`.
- [ ] A test pins the order of a mission whose items are in several states in `show <mission>` (from atelier-31l2).
- [ ] A test with more than 20 ready items shows every row in `list --ready` (from atelier-8zxh).

## Scope

Ordering only; no change to which rows are shown.

## Context

Rewritten for the new tool in the cutover sort (atelier-5zre): `work blocked` and `work mission` became `list --blocked` and `show <mission>`, and `work all` became `list`; the old functions and the shared order doc are gone, so the rule is stated in the Done when.
