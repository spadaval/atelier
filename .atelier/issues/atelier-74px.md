---
created_at: "2026-06-25T16:23:24.306065759+00:00"
id: "atelier-74px"
issue_type: "validation"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "done"
title: "Validate work dashboard migration"
updated_at: "2026-06-25T16:23:24.306065759+00:00"
---

## Description

Prove `work queue`, `work mission <id>`, and `work epic <id>` cover the useful current `issue list` behavior plus mission-level grouping. Focused tests should cover mission/epic/issue nesting, orphaned work, ready/active/blocked/backlog filters, quiet output, `--all` or equivalent audit scope if implemented, and removed `issue list` rejection.

## Outcome

Focused validation proves the work dashboard migration preserves useful queue behavior, adds mission-level grouping, supports state filters and quiet output, handles orphaned work, and rejects removed `issue list` without compatibility aliases. Evidence records command transcripts and any issue-linked exceptions.
