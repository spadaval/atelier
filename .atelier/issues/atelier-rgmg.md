---
created_at: "2026-06-17T18:00:01.688899039+00:00"
id: "atelier-rgmg"
issue_type: "task"
labels:
- "fields"
- "records"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T00:38:49.273833726+00:00"
status: "done"
title: "Add canonical issue field parsing, rendering, validation, and projection support"
updated_at: "2026-06-18T00:38:49.273833726+00:00"
---

## Description

Add canonical issue-field support through RecordStore and rebuild/lint paths so
typed fields are durable Markdown state.

## Outcome

- Issue records parse, render, and preserve a `fields` mapping.
- Lint and rebuild validate field values against workflow schema version 2.
- Projection/query paths expose enough field data for PR validators and status
  surfaces.

## Evidence

- Focused RecordStore and rebuild tests prove issue field parse/render,
  validation failure, and projection recovery behavior.
- Command transcript shows targeted tests and `atelier export --check` pass.
