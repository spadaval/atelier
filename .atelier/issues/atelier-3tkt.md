---
created_at: "2026-06-11T20:09:59.529402818+00:00"
id: "atelier-3tkt"
issue_type: "epic"
labels:
- "docs"
- "markdown"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-29oi"
  - kind: "issue"
    id: "atelier-7n3w"
  - kind: "issue"
    id: "atelier-pjai"
  children:
  - kind: "issue"
    id: "atelier-33lr"
  - kind: "issue"
    id: "atelier-81q1"
  - kind: "issue"
    id: "atelier-q5lk"
  - kind: "issue"
    id: "atelier-veof"
  attachments:
  - kind: "evidence"
    id: "atelier-35xf"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:13:05.117383453+00:00"
status: "done"
title: "Specify the Markdown-first single-tree contract"
updated_at: "2026-06-11T23:13:05.117383453+00:00"
---

## Description

Document the Markdown-first storage contract before implementation. This epic owns the product and architecture language for the single .atelier/ layout, direct edit workflow, deterministic Markdown shape, activity/notes model, migration/backcompat window, merge-conflict guidance, and validation responsibilities.
- PRODUCT_INTENT.md, CONTEXT.md, AGENTFACTORY.md, storage docs, and ADR language describe .atelier/ as the committed canonical root with ignored runtime/cache subpaths.
- The direct-edit contract defines deterministic front matter ordering, required fields, relationship buckets, activity sidecar rules, and examples for creating/modifying records by hand.
- The compatibility window for .atelier-state is explicit: discover/read/migrate only, write only the new layout after migration.
- Merge-conflict and hand-edit recovery guidance is documented for operators and agents.
- Follow-on implementation issues link back to this contract instead of inventing path or validation policy locally.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
