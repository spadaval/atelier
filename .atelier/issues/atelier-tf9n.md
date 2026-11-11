---
created_at: "2026-10-17T01:10:57.984299499+00:00"
id: "atelier-tf9n"
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
    id: "atelier-4pzj"
    type: "advances"
  - kind: "review"
    id: "atelier-wfan"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show direct blockers on work rows, mark inherited ones, drop the count"
updated_at: "2026-10-17T01:10:57.984299499+00:00"
---

## Goal

`atelier list` and `show` name each row's direct blockers with their status, but a row whose blocker is itself blocked does not say where its chain ends: with atelier-1a5k blocked by atelier-1ci8, itself blocked by atelier-5qes (links set for the test in a scratch clone), the row for atelier-1a5k shows only `blocked by atelier-1ci8, which is draft` (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] A row in `list` or `show <mission>` whose chain is longer than 1 names its far end: `(chain of 3, ends at atelier-ubti)`.
- [ ] A test pins `list` and `show <id>` to the same direct blocker IDs for the same item.

## Context

Reworks the same row rendering as atelier-deyy. Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `list` already names each row's direct blockers and prints no blocker count or "readiness gate", and parents and the `work` views are gone, so only the chain's far end remains; `work mission`, `work blocked` and `issue show` became `list` and `show`.
