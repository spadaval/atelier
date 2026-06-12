---
created_at: "2026-06-12T05:11:53.903149935+00:00"
id: "atelier-u6ax"
issue_type: "task"
labels:
- "evidence"
- "lint"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-w8e3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Enforce concrete Evidence sections in lint"
updated_at: "2026-06-12T05:11:53.903149935+00:00"
---

## Description

Teach lint to reject work items whose Evidence section is missing, empty, or too
vague to guide validation.

## Outcome

- Lint fails when an executable issue lacks a non-empty Evidence section.
- Lint flags Evidence entries that do not name an observable proof target, such
  as a command, transcript, evidence record, test, review artifact, file change,
  or manual check.
- Lint distinguishes optional notes from required evidence expectations.
- Lint diagnostics name the issue ID, section, and file path to edit.

## Evidence

- Lint tests cover missing Evidence, empty Evidence, vague Evidence, and concrete

Evidence.

- CLI transcript tests show actionable lint diagnostics.

- Run focused lint tests.
