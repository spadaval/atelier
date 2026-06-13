---
created_at: "2026-06-12T19:29:26.567235806+00:00"
id: "atelier-6aor"
issue_type: "validation"
labels:
- "mission"
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-gjaz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T22:28:56.925326255+00:00"
status: "done"
title: "Validate readable mission record workflow end to end"
updated_at: "2026-06-12T22:28:56.925326255+00:00"
---

## Description

Validate the readable mission record workflow after the contract,
implementation, relationship cleanup, and migration slices land.

## Outcome

- Positive command transcripts cover mission create, update, show, status,
  rebuild, export/check, lint, and doctor on the new record shape.
- Negative tests or transcripts cover invalid mission records and any stale code
  path that would still emit escaped mission `data` JSON.
- Validation reviews the migrated current mission and confirms the record can be
  audited without private context.
- Residual risks are named before this validation issue closes.

## Evidence

- Focused test output for mission record parsing, rendering, migration, and
  relationship behavior.
- Command transcripts for the representative positive and negative paths.
- Linked validation notes covering the current mission record after migration.
