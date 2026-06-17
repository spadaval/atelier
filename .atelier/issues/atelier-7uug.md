---
created_at: "2026-06-17T20:04:15.951534521+00:00"
id: "atelier-7uug"
issue_type: "task"
labels:
- "implementation"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kvxp"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T22:28:35.523404698+00:00"
status: "done"
title: "Move workflow parsing and transition semantics into atelier-workflow"
updated_at: "2026-06-17T22:28:35.523404698+00:00"
---

## Description

Move workflow policy parsing and transition semantics into `atelier-workflow`.
Current live behavior is spread across app and CLI modules, making the dedicated
workflow crate mostly a starter-policy holder rather than the semantic owner.

## Outcome

- `atelier-workflow` exposes cohesive APIs for policy parsing, status category
  resolution, transition lookup, validator lookup, and readiness inputs.
- `atelier-app` calls those APIs instead of maintaining separate policy logic.
- CLI modules do not interpret workflow policy directly except for rendering
  already-computed results.

## Evidence

- Unit tests in `atelier-workflow` cover policy parsing, categories, transitions,
  and invalid policy files.
- Search transcript proves duplicated app/CLI parsing and transition logic is
  removed or reduced to adapters.
