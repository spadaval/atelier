---
created_at: "2026-06-17T19:37:52.466199572+00:00"
id: "atelier-adub"
issue_type: "task"
labels:
- "records"
- "refactor"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Split RecordStore by durable record ownership"
updated_at: "2026-06-17T19:37:52.466199572+00:00"
---

## Description

Split the large RecordStore implementation by durable ownership for the
supported v1 record model instead of continuing to concentrate issue, mission,
evidence, document, front matter, validation, and store-entrypoint behavior in
one module. First-class plan and milestone record removal belongs to the
parallel removal epic; do not preserve or refactor those records here except as
needed to delete obsolete references safely.

## Outcome

- RecordStore has separate modules for supported canonical record ownership:
  issue, mission, evidence, document/front matter, validation, and store
  entrypoint behavior.
- Public records APIs preserve the existing canonical Markdown format unless a
  linked contract task changes it.
- Callers do not need to duplicate record-kind lists, relationship constructors,
  or rendering details.
- First-class plan and milestone record behavior is not made more permanent by
  this split; any touched obsolete code is either left to the removal epic or
  deleted when that deletion is local and safe.

## Evidence

- Focused RecordStore round-trip tests cover supported records: issues,
  missions, evidence, relationships, and activity sidecars.
- File review shows the top-level records module is an entrypoint instead of a
  mixed parser/renderer/validator implementation.
- Search or diff review proves the split did not add new first-class plan or
  milestone ownership surfaces.
- `atelier export --check` and targeted records tests pass.
