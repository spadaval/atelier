---
created_at: "2026-11-10T00:56:09.647724857+00:00"
id: "atelier-5zre"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Sort the 147 open items and five open missions for the new tool"
updated_at: "2026-11-10T00:56:09.647724857+00:00"
---

## Goal

Decide, for every open record, whether it converts as it is, is rewritten for the new tool, or is dropped as obsolete.

## Done when

- [ ] Provisional: a list with one line per open record and its fate, applied to the records, with the dropped ones named in the Report.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- How many of the 57 `cli-output` items the new commands make moot.
