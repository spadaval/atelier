---
created_at: "2026-06-13T20:35:24.123657925+00:00"
id: "atelier-p2m2"
issue_type: "epic"
labels:
- "cli"
- "stabilization"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2rf7"
  - kind: "issue"
    id: "atelier-5a73"
  - kind: "issue"
    id: "atelier-i9ob"
  - kind: "issue"
    id: "atelier-rgd1"
  - kind: "issue"
    id: "atelier-u08r"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Make the CLI surface purposeful and compact"
updated_at: "2026-06-13T21:58:03.428554959+00:00"
---

## Description

Audit and simplify the visible and hidden command surface so every command has a distinct operator job and default output contains only the information needed for that job.
- Primary help and group help expose a small, coherent workflow surface.
- Duplicate predecessor verbs and compatibility entrypoints are removed, hidden only when explicitly justified, or replaced by one documented command.
- Default output for common commands answers the immediate operator question without leaking diagnostics or stale migration language.
- Help transcript review compares top-level, issue, mission, evidence, worktree, health, and relationship groups before and after the pass.
- Residue search or tests prove removed command names fail or are absent from normal help.
- Focused CLI workflow transcripts prove drill-down arguments still expose needed detail.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
