---
created_at: "2026-06-17T17:59:59.203859627+00:00"
id: "atelier-nmkm"
issue_type: "task"
labels:
- "fields"
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
title: "Support workflow schema version 2 typed field definitions"
updated_at: "2026-06-17T17:59:59.203859627+00:00"
---

## Description

Extend workflow policy parsing to schema version 2 with top-level typed field
definitions for issue records.

## Outcome

- Workflow schema version 2 accepts `fields` definitions.
- Supported field types are string, bool, integer, enum, and object with
  required keys.
- Unknown field-definition shapes produce strict workflow configuration errors.

## Evidence

- Focused unit tests cover valid and invalid workflow schema version 2 field
  definitions.
- Command transcript shows the targeted workflow-policy test filter passes.
