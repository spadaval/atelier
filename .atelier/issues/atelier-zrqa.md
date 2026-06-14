---
created_at: "2026-06-14T05:58:54.055426552+00:00"
id: "atelier-zrqa"
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
title: "Expand graph commands to missions"
updated_at: "2026-06-14T08:19:30.374995234+00:00"
---

## Description

Expand graph impact and tree from issue-only views to cross-record mission and issue relationship inspection.

## Outcome

`atelier graph impact` and `atelier graph tree` inspect cross-record
relationships that include missions and issues. Help text no longer describes
graph as issue-only when mission relationships are in scope.

## Evidence

- `atelier graph --help`, `graph impact --help`, and `graph tree --help`
  describe cross-record mission/issue graph behavior.
- Focused tests or transcripts cover a mission linked to an epic/issue and show
  graph output including mission and issue nodes.
- Focused graph tests or docs diff show existing issue-only graph behavior
  remains covered or is intentionally reclassified.
- `git diff --check`, `atelier lint`, and focused graph tests pass.
