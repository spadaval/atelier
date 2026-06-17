---
created_at: "2026-06-17T20:03:26.864300628+00:00"
id: "atelier-a3e7"
issue_type: "task"
labels:
- "implementation"
- "plans"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-aqqc"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove plan record CRUD and canonical plan storage"
updated_at: "2026-06-17T20:03:26.864300628+00:00"
---

## Description

Remove first-class plan record behavior. This includes plan create/show/list,
revise, link, canonical plan parsing/rendering, projection tables, fixtures, and
tests that treat plans as durable tracker records.

## Outcome

- `atelier plan create`, `show`, `list`, `revise`, and `link` are removed from
  the public command surface.
- `.atelier/plans/` is not created by init and is not accepted as a canonical
  rebuild source.
- Record kind registration, parser/rendering code, projection schema, command
  help, man pages, and tests no longer expose first-class plan records.
- Existing references to plan Markdown use ordinary file links or prose in
  accountable work records.

## Evidence

- Help transcripts prove removed plan-record commands are absent or rejected.
- Focused tests prove rebuild/lint reject or ignore unsupported plan-record
  inputs according to the settled contract.
- Search transcript proves production code no longer creates plan records.
