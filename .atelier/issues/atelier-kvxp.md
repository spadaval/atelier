---
created_at: "2026-06-17T20:04:18.545961354+00:00"
id: "atelier-kvxp"
issue_type: "validation"
labels:
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate CLI readiness uses workflow APIs instead of local policy copies"
updated_at: "2026-06-17T20:04:18.545961354+00:00"
---

## Description

Validate that CLI readiness, status, and transition views consume shared
workflow APIs after the boundary repair. This is independent validation of the
workflow ownership epic.

## Outcome

- `atelier issue transition --options`, `atelier status`, workflow validation,
  and mission readiness produce the same user-facing behavior after migration.
- Source review confirms CLI code renders workflow results instead of carrying
  separate readiness semantics.

## Evidence

- Command transcripts cover representative transition options, blocked issue,
  close-ready issue, invalid workflow YAML, and mission status behavior.
- Search transcript proves CLI/app workflow duplication is gone.
- Focused tests and `atelier lint` pass.
