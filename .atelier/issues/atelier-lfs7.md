---
created_at: "2026-10-17T17:25:59.636065900+00:00"
id: "atelier-lfs7"
issue_type: "validation"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-rx22"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Validate a real mission's HTML debrief with the human"
updated_at: "2026-10-17T17:25:59.636065900+00:00"
---

## Goal

S7 of docs/strategies/review-and-introspection.md: a real mission that ran with decision capture and reports switched on produces an HTML debrief that the human judges good enough for post-mission review.

## Done when

The human opens the debrief in T3 Code and records a review event naming the commit it was generated from.

## Unknowns

Which mission. The strategy names the orientation mission (atelier-1w8c), but 5 of its 7 items closed before S1 and S2 can land, so it may no longer be a fair test. Choosing another candidate changes S7 and needs the human.
