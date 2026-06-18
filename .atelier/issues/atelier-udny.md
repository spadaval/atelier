---
created_at: "2026-06-17T18:00:43.374903240+00:00"
id: "atelier-udny"
issue_type: "task"
labels:
- "pr"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T01:03:12.342988973+00:00"
status: "done"
title: "Render live unresolved inline PR comments without mirroring bodies"
updated_at: "2026-06-18T01:03:12.342988973+00:00"
---

## Description

Render unresolved inline PR review comments as implementer-facing work while
keeping Forgejo as the source of truth for comment bodies.

## Outcome

- `atelier pr comments --unresolved` lists actionable unresolved inline
  comments grouped by file or thread.
- Output includes enough ID/path/line context for implementers to respond.
- Atelier records only high-level activity, not mirrored full comment bodies.

## Evidence

- Mocked Forgejo test covers unresolved inline comment rendering and resolved
  comment filtering.
- Command transcript shows targeted PR comments tests pass.
