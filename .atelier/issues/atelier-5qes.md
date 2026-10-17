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
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Order work-view rows with the shared blocker-first order"
updated_at: "2026-10-17T01:10:57.983995411+00:00"
---

## Goal

`docs/product/work-view-ordering.md` says `work ready`, `work blocked`, `work mission` and `work parent` show a visible blocker before what it blocks, then sort by state, priority, update time and ID. Only `issue show` calls `ordered_work_indices` (crates/atelier-cli/src/commands/work_order.rs:8, used at issue.rs:404). Instead:
- `work blocked` uses `sort_work_rows` (priority, status, ID; work_classification.rs). The fixture lists `atelier-5hp9 … 3 blockers` above atelier-fhep and atelier-il31, which block it.
- `work all` re-sorts blocked rows by ID (crates/atelier-cli/src/commands/work.rs:198-200), so on master `work blocked` prints `sdb0, 7u4o, deyy` and `work all` prints `7u4o, deyy, sdb0`.
- Mission and parent rows follow `issue_descendant_ids`, i.e. ID order; in `work mission 1w8c` the high-priority sdb0 is last among blocked rows.

## Done when

- Every work view passes its chosen rows through `ordered_work_indices`.
- `work blocked` and `work all` print blocked rows in the same order.
- A test with chain ubti → il31 → fhep → 5hp9 asserts that order in `work blocked` and `work mission`.

## Scope

Ordering only; no change to which rows are shown.
