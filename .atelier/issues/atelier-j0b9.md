---
created_at: "2026-11-05T04:50:24.448397240+00:00"
id: "atelier-j0b9"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-gg6b"
    type: "advances"
  - kind: "issue"
    id: "atelier-mgl0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Settle planned-scope edge cases in the debrief"
updated_at: "2026-11-05T04:50:24.448397240+00:00"
---

## Goal

Make the debrief's notion of planned work consistent at its edges, which predate atelier-mgl0's deferral rule.

## Done when

Each case below shows one consistent place in the debrief, with a fixture test:
- A child of an item added after the start is in scope and not added_after_start, so a deferral to it is skipped as planned, yet it is not listed as planned and lands in Left to do without a source (record_history.rs added_after_start only records children of parents in scope at the start).
- A planned item that also declares discovered_from a scoped item is shown as found, so the comment that a deferral wins over discovery no longer holds for planned targets.
- An item planned at the start and later unlinked counts as filed when a finding is deferred to it; the product doc does not describe this.
