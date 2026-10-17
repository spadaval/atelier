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
status: "todo"
title: "Show the current decisions on an item in status and issue show"
updated_at: "2026-10-17T18:17:06.615557096+00:00"
---

## Goal

S6 of docs/strategies/review-and-introspection.md. An agent working an item should see the decisions that bear on it, through its parent and mission, without searching.

## Done when

- `issue show <id>` (brief) prints `Decisions: <n> current` with the one-way decisions as rows beneath, and `issue show <id> --full` prints a Decisions section with every current decision. The decisions are the decision notes on the item, its parent and its mission record, and the current decision records in the item's mission scope; records that are open or superseded are not current.
- `status` shows the same brief form for the owner it is working on.
- Rows use the decision row and item line shapes from docs/product/human-cli-output.md, each naming the record it came from.
- CLI tests cover the brief and full views and `status`, and docs/product/human-cli-output.md and sessions-and-hooks.md describe them.

## Unknowns

How much this adds to `status` output on a mission with many decisions (ADR 0022's density rules apply).
