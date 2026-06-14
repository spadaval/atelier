---
created_at: "2026-06-11T20:10:55.605429835+00:00"
id: "atelier-jarw"
issue_type: "task"
labels:
- "assignee:root"
- "lint"
- "markdown"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ca32"
  - kind: "issue"
    id: "atelier-iw2l"
  - kind: "issue"
    id: "atelier-qrix"
  - kind: "issue"
    id: "atelier-unma"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-67du"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:41:24.065772739+00:00"
status: "done"
title: "Validate canonical .atelier Markdown directly in atelier lint"
updated_at: "2026-06-11T23:41:24.065772739+00:00"
---

## Description

Make lint parse committed .atelier/ records directly rather than trusting SQLite projection state. Acceptance: deleting state.db does not prevent canonical lint from validating committed records.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
