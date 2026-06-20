---
created_at: "2026-06-19T22:42:56.473630390+00:00"
id: "atelier-cko9"
issue_type: "task"
labels:
- "actions"
- "schema"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0d5k"
  - kind: "issue"
    id: "atelier-pv77"
  - kind: "issue"
    id: "atelier-z7vb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T00:03:48.543637390+00:00"
status: "done"
title: "Rename transition effects to actions"
updated_at: "2026-06-20T00:03:48.543637390+00:00"
---

## Description

Rename workflow `effects` to `actions` in the schema, docs, parser, diagnostics, and transition option output. Use namespaced action names and parameter objects where needed.

## Outcome

- Workflow transitions use `actions` as the committed schema and CLI vocabulary.
- Legacy `effects` syntax is rejected rather than retained as a compatibility alias.

## Evidence

- `target/debug/atelier workflow check` rejects `effects` and accepts `actions`.
- CLI integration tests assert transition output says Planned Actions.
- Tests cover invalid action names and invalid action params.
