---
created_at: "2026-06-13T20:37:05.319570151+00:00"
id: "atelier-of3h"
issue_type: "task"
labels:
- "assignee:root"
- "data-model"
- "evidence"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Replace evidence escaped data JSON with typed Markdown fields"
updated_at: "2026-06-13T22:55:59.053327382+00:00"
---

## Description

Evidence records currently store normal metadata in an escaped data JSON scalar, which makes canonical Markdown hard to read and hides logical fields. Replace that shape with typed front matter and/or body sections defined by the record contract.

## Outcome

- Evidence metadata such as kind, result, producer, proof scope, independence level, residual risks, follow-up IDs, path, URI, captured command, and bounded output summaries has readable canonical placement.
- Existing evidence records migrate deterministically without losing attachments or proof summaries.
- Evidence show/list/record/attach behavior uses the typed fields rather than generic data_json plumbing.

## Evidence

- Migration file change or fixture file artifact shows before/after evidence Markdown without escaped data JSON.
- `atelier rebuild`, `atelier export --check`, `atelier lint`, and focused evidence command test transcripts pass.
- `rg` command output residue search proves normal evidence rendering no longer writes frontmatter data as escaped JSON.
