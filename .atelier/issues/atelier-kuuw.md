---
created_at: "2026-06-19T03:58:20.286519497+00:00"
id: "atelier-kuuw"
issue_type: "task"
labels:
- "docs"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Record ADR for native review modes and room authority"
updated_at: "2026-06-19T03:58:20.286519497+00:00"
---

## Description

Record the durable architecture decision for native review rooms and the
renamed review command surface. The ADR should supersede the provider-first PR
assumption from ADR 0010 where needed without preserving compatibility shims.

## Outcome

- A new ADR defines `review.mode = "room"` and provider-backed
  `review.mode = "provider"` as mutually exclusive project modes.
- The ADR specifies `.atelier/reviews/<id>.yaml` as the canonical room record,
  with current room state derived from metadata plus ordered events.
- The ADR decides the new structured issue `review` field, migration away from
  legacy `pull_request`, the `atelier review ...` command surface, and room
  merge authority.
- Rejected alternatives cover PR command aliases, duplicate room snapshots, PR
  style inline thread lifecycle, and adding broader GitHub/GitLab provider
  abstraction in this mission.

## Evidence

- File diff for the new ADR under `docs/adr/` contains context, decision,
  consequences, and rejected alternatives.
- File diff for existing ADR references that describe provider-first PR state
  shows they are updated or explicitly contextualized.
- `rg -n 'atelier pr|pull_request|review.mode|review merge' docs/adr CONTEXT.md`
  output is reviewed and recorded in evidence.
