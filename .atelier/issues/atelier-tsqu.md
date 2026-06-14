---
created_at: "2026-06-14T02:52:05.915838243+00:00"
id: "atelier-tsqu"
issue_type: "task"
labels:
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Add proof-routing examples for notes evidence and validation"
updated_at: "2026-06-14T08:27:56.508998308+00:00"
---

## Description

Add examples that contrast issue notes, first-class evidence, command-backed
evidence, and independent validation evidence after the evidence capture and
closeout gates are stable.

## Outcome

Workers can choose the right proof surface for ordinary issues, process-policy changes, broad validation, and closeout claims.

## Evidence

- File diff in validation or work-model docs includes examples tied to
  Outcome/Evidence requirements.
- File diff or review artifact shows when an issue note is enough, when
  first-class evidence is required, when command-backed evidence is appropriate,
  and when independent validation evidence is required.
- `git diff --check` and `atelier lint` pass.
