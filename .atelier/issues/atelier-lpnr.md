---
created_at: "2026-06-13T20:35:25.565402990+00:00"
id: "atelier-lpnr"
issue_type: "epic"
labels:
- "data-model"
- "markdown"
- "stabilization"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-fyrm"
  - kind: "issue"
    id: "atelier-nqp4"
  - kind: "issue"
    id: "atelier-of3h"
  - kind: "issue"
    id: "atelier-x45p"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Normalize canonical Markdown and record data"
updated_at: "2026-06-13T20:37:08.560368636+00:00"
---

## Description

Turn the canonical record formats into a small, readable data model with logical fields, typed sections, and deterministic rendering. Remove generic or duplicated fields where they make records harder to understand.

## Outcome

- Issue, mission, plan, evidence, and activity records have documented field ownership and section placement.
- Normal records do not rely on escaped generic JSON payloads for first-class fields.
- Runtime/cache configuration matches the single-tree product contract and does not blur canonical records with local state.

## Evidence

- Data-model contract artifact lists the canonical fields and sections for each record kind.
- Representative Markdown fixture diff proves records are readable, deterministic, and rebuildable.
- atelier lint, atelier export --check, rebuild scenario, and focused parser tests pass.
