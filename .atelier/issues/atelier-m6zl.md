---
created_at: "2026-06-17T19:38:15.898621682+00:00"
id: "atelier-m6zl"
issue_type: "task"
labels:
- "architecture"
- "docs"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2573"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Decide comment sidecar API ownership"
updated_at: "2026-06-17T23:05:31.744857379+00:00"
---

## Description

Decide and record where comment sidecar APIs belong. Current SQLite methods read
and write canonical activity sidecars, which blurs the projection boundary even
though the sidecars are canonical Markdown files.

## Outcome

- A durable architecture note or docs update states whether comment sidecar APIs
  belong in `atelier-records` or remain as an app-level adapter.
- The chosen boundary explains how import, issue note commands, history, and
  tests should access activity sidecars.
- Dependent SQLite cleanup work has an unambiguous target boundary.

## Evidence

- Documentation diff or evidence record states the decision and maps it to the
  affected callers.
- Search transcript lists current comment sidecar callers and classifies their
  destination boundary.
- `atelier lint` and `git diff --check` pass.
