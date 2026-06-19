---
created_at: "2026-06-19T03:58:22.932154053+00:00"
id: "atelier-cix4"
issue_type: "task"
labels:
- "docs"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:19:06.189492575+00:00"
status: "done"
title: "Update domain, product, workflow, and CLI docs for review modes"
updated_at: "2026-06-19T04:19:06.189492575+00:00"
---

## Description

Update durable product and operator documentation so future implementation work
has one source of truth for review rooms and review commands.

## Outcome

- `CONTEXT.md` defines review mode, review room, provider-backed review
  artifact, room event, finding, approval, and merge authority using the new
  terminology.
- Product workflow and validation docs describe the `review` issue field,
  child inheritance from parent epics, old-field rejection, and the separation
  between `review merge` and issue workflow transitions.
- CLI surface docs list `atelier review open/status/show/merge/comments/comment`,
  `approve`, `request-changes`, `resolve`, and provider-only `link`, and remove
  normal guidance for `atelier pr`.
- Starter workflow guidance names the room and provider modes without teaching
  compatibility aliases.

## Evidence

- File diff includes the changed domain, product, validation, workflow, and CLI
  surface documentation files.
- Search output proves current docs/help source text no longer presents
  `atelier pr` as an active command.
- `git diff --check -- '*.md'` and `atelier lint atelier-cix4` pass.
