---
created_at: "2026-06-11T20:10:52.813630582+00:00"
id: "atelier-pgkd"
issue_type: "task"
labels:
- "migration"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dnsx"
  - kind: "issue"
    id: "atelier-ru15"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-vo53"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add one-shot markdown-first layout migration"
updated_at: "2026-06-11T23:31:47.652647633+00:00"
---

## Description

Implement atelier migrate markdown-first or equivalent internal migration to move existing committed records from .atelier-state/* into .atelier/*, preserve runtime state under ignored paths, and report clear next steps. Acceptance: writes after migration target only the new canonical layout.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
