---
created_at: "2026-06-14T03:47:15.649260323+00:00"
id: "atelier-j01c"
issue_type: "task"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Auto-refresh projection caches before operator-facing commands"
updated_at: "2026-06-14T03:47:15.649260323+00:00"
---

## Description

Manual canonical edits and tracker mutations should not require operators to think about rebuild/export freshness. Operator-facing reads and health commands should transparently refresh rebuildable projection/cache state when canonical records are valid, while still surfacing invalid canonical Markdown clearly.

## Outcome

After a direct canonical Markdown edit, the next normal Atelier command refreshes projection/cache state automatically or reports only a true canonical validation problem, aligning with the Zen principle that operators reason about work rather than implementation mechanics.

## Evidence

Focused test or transcript edits a canonical issue/mission file, then runs an operator-facing command without manual `atelier rebuild`; output is current and `atelier export --check` passes afterward.
