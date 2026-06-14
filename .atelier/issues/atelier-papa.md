---
created_at: "2026-06-14T02:52:33.624662470+00:00"
id: "atelier-papa"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Make evidence relation errors corrective"
updated_at: "2026-06-14T07:31:34.069251142+00:00"
---

## Description

Improve evidence attach/link relation errors so invalid roles like validation name the accepted vocabulary and the normal evidence target flow.

## Outcome

Agents can correct invalid evidence relation attempts without guessing role names.

## Evidence

- Focused CLI tests or transcripts cover an invalid evidence relation such as
  `validation`, show the accepted relation vocabulary, and name the normal
  target-and-attach evidence flow.
- Focused CLI test or transcript records or attaches evidence using the
  accepted positive path.
- `git diff --check`, `atelier lint`, and focused evidence CLI tests pass.
