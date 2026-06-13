---
created_at: "2026-06-13T20:44:45.160859105+00:00"
id: "atelier-1krs"
issue_type: "validation"
labels:
- "readiness"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1vdl"
  - kind: "issue"
    id: "atelier-1xuf"
  - kind: "issue"
    id: "atelier-c9ej"
  - kind: "issue"
    id: "atelier-e723"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Record agent-readiness audit and score"
updated_at: "2026-06-13T20:44:45.160859105+00:00"
---

## Description

Perform a focused agent-readiness audit using Agent Factory readiness criteria plus applicable Factory-style criteria such as dev environment reproducibility, docs validation, quality tooling, ownership templates, task discovery, dependency hygiene, and secrets/env onboarding.

## Outcome

- The audit classifies each applicable criterion as pass, fail, skipped, or deferred with file or command evidence.
- The audit identifies which gaps are already covered by stabilization issues and which need new owner issues.
- The resulting score and recommendations are durable enough for a future closeout auditor to inspect without private chat context.

## Evidence

- Evidence record or review artifact captures the criterion table, inspected files, and command transcripts.
- `atelier search`, `find`, and `rg` command output demonstrates duplicate/follow-up checks before new work is added.
- `atelier lint` and `atelier export --check` command transcripts pass after tracker updates.
