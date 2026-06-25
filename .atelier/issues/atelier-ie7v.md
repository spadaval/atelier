---
created_at: "2026-06-19T18:25:37.376397755+00:00"
id: "atelier-ie7v"
issue_type: "epic"
labels:
- "artifact-update"
- "review"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qsmn"
  - kind: "issue"
    id: "atelier-zr6j"
  children:
  - kind: "issue"
    id: "atelier-8cjb"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T20:49:57.054202265+00:00"
status: "done"
title: "Epic: Product contract and ADR for workflow transition effects"
updated_at: "2026-06-19T20:49:57.054202265+00:00"
---

## Description

Settle the product and architecture contract for transition effects before
schema, engine, or CLI work starts. This epic owns the ADR, product docs, and
domain language that explain why effects belong to explicit workflow
transitions and why review commands remain review-artifact operations.

## Outcome

- `PRODUCT_INTENT.md`, `CONTEXT.md`, product docs, and a new or amended ADR define
  transition effects as configured, previewable work performed by successful
  issue transitions.
- The docs distinguish validators from effects: validators gate a transition;
  effects run only after readiness succeeds and have named failure/recovery
  behavior.
- The v1 effect set is intentionally small and includes explicit review
  artifact opening/linking while excluding review merge, issue close, hidden PR
  aliases, or provider actions that bypass workflow policy.
- Dependent schema, engine, CLI, and validation work can proceed without
  guessing how review integration should behave.

## Evidence

- Documentation diff maps the effect lifecycle, review boundary, and v1 effect
  set to mission validation criteria.
- Search transcript shows old guidance does not imply review commands transition
  issues or that provider PR actions replace workflow transitions.
- `atelier lint atelier-ie7v` passes after the child artifact update lands.
