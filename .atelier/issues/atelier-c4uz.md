---
created_at: "2026-06-14T16:31:01.342764800+00:00"
id: "atelier-c4uz"
issue_type: "task"
labels:
- "assignee:root"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T17:23:15.554356882+00:00"
status: "done"
title: "Remove heuristic evidence text matching from close gates"
updated_at: "2026-06-14T17:23:15.554356882+00:00"
---

## Description

Closeout gates should not decide proof sufficiency by token overlap against free-form Outcome or Evidence prose.

## Outcome

- Core closeout validators do not call the heuristic that matches evidence
  records to free-form requirement text by token overlap.
- Evidence text can still be displayed for operators, but it is not the
  authoritative proof-sufficiency gate.
- Any remaining text-matching helper is removed or demoted to an advisory
  diagnostic that cannot block a close transition.

## Evidence

- `rg` transcript or code review artifact shows closeout gates no longer
  depend on heuristic `Outcome`/`Evidence` token matching.
- Focused workflow tests or transcripts prove a close transition is governed by
  explicit workflow/evidence state rather than matching phrasing in an evidence
  summary.
- `git diff --check` and `atelier lint` pass.
