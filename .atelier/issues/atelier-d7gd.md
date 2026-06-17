---
created_at: "2026-06-17T17:59:47.067548016+00:00"
id: "atelier-d7gd"
issue_type: "task"
labels:
- "context"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:50:17.117119906+00:00"
status: "done"
title: "Update CONTEXT for Session, Typed field, and Pull request artifact"
updated_at: "2026-06-17T23:50:17.117119906+00:00"
---

## Description

Update the domain glossary so future agents use the new terms precisely and do
not confuse durable sessions with removed legacy runtime session commands.

## Outcome

- `CONTEXT.md` defines Session, Typed field, and Pull request artifact.
- The ambiguity notes distinguish durable session records from current-work
  source of truth and from local command diagnostics.
- Relationships among sessions, issues, epics, PR artifacts, and validators are
  clear enough for implementation agents.

## Evidence

- File content check over `CONTEXT.md` proves the new terms and ambiguity
  notes are present.
- Command transcript shows `git diff --check -- CONTEXT.md` passes.
