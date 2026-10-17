---
created_at: "2026-10-17T17:25:59.261604774+00:00"
id: "atelier-1mk4"
issue_type: "feature"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fip8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show the current decisions on an item in status and issue show"
updated_at: "2026-10-17T17:25:59.261604774+00:00"
---

## Goal

S6 of docs/strategies/review-and-introspection.md. An agent working an item should see the decisions that bear on it, through its parent and mission, without searching.

## Done when

`status` (for the owner in progress) and `issue show` list the current decisions on the item, its parent and its mission. The brief view shows the count and the one-way decisions, and the full view shows them all. Covered by CLI tests.

## Unknowns

How much this adds to `status` output on a mission with many decisions (ADR 0022's density rules apply).
