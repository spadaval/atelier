---
created_at: "2026-06-17T20:03:43.547300899+00:00"
id: "atelier-mwup"
issue_type: "epic"
labels:
- "architecture"
- "audit"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-98mo"
  children:
  - kind: "issue"
    id: "atelier-7uug"
  - kind: "issue"
    id: "atelier-kvxp"
  - kind: "issue"
    id: "atelier-yq99"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T22:32:46.314662578+00:00"
status: "done"
title: "Reconcile workflow semantics with atelier-workflow crate ownership"
updated_at: "2026-06-17T22:32:46.314662578+00:00"
---

## Description

Reconcile workflow ownership with the crate architecture. The architecture says
workflow parser, status categorization, transition rules, and validator semantics
belong in `atelier-workflow`, but the audit found most live behavior still in
`atelier-app` and CLI command modules.

This is not a request to add new workflow behavior. It is boundary repair:
workflow semantics should have one implementation surface that app and CLI code
consume.

## Outcome

- The product/architecture contract states which workflow responsibilities live
  in `atelier-workflow`, `atelier-app`, and `atelier-cli`.
- Workflow policy parsing, status-category handling, transition lookup, and
  readiness semantics are exposed through cohesive `atelier-workflow` APIs.
- `atelier-app` orchestrates use cases with those APIs instead of duplicating
  workflow interpretation.
- CLI command modules render workflow results and no longer carry separate
  policy semantics.

## Evidence

- Search transcript proves workflow parsing and transition semantics are not
  duplicated across CLI/app modules.
- Focused tests cover starter policy parsing, status categories, transition
  options, blocked readiness, close readiness, and invalid workflow files through
  the shared workflow APIs.
- Representative CLI transcripts remain behaviorally stable for
  `atelier issue transition --options`, `atelier status`, and
  `atelier mission status`.
