---
created_at: "2026-06-17T19:38:00.579999845+00:00"
id: "atelier-3h90"
issue_type: "task"
labels:
- "data-model"
- "records"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7g43"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unify canonical priority representation"
updated_at: "2026-06-17T19:38:00.579999845+00:00"
---

## Description

Unify priority handling so canonical Markdown, human CLI labels, SQLite
projection filters, and work-order sorting cannot drift. Current docs define
canonical `P0` through `P3`, while some code still accepts or stores
`critical`, `high`, `medium`, and `low` as durable values.

## Outcome

- A single core priority type or conversion API owns `P0`/`P1`/`P2`/`P3` and
  human labels `critical`/`high`/`medium`/`low`.
- Canonical Markdown writes durable priority tokens according to the documented
  contract.
- CLI parsing, projection filtering, import mapping, and work-order sorting use
  the shared conversion API.
- Invalid priority input produces one consistent corrective error shape.

## Evidence

- Focused tests cover canonical parse/render, CLI label input, projection
  filter behavior, import mapping, sorting rank, and invalid input.
- Search transcript shows duplicate priority vocabularies or local rank tables
  have been removed or delegated to the shared API.
- `atelier lint` and targeted records/sqlite/CLI tests pass.
