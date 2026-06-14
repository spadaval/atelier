---
created_at: "2026-06-14T02:52:02.984801054+00:00"
id: "atelier-r5tr"
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
status: "todo"
title: "Document canonical projection recovery model"
updated_at: "2026-06-14T02:52:02.984801054+00:00"
---

## Description

Document the difference between canonical Markdown writes and projection refresh, plus the normal repair order for stale projections, invalid canonical Markdown, and runtime/cache artifacts.

## Outcome

Agents can tell whether a durable write landed and what to do when projection freshness blocks reads or workflow commands.

## Evidence

Docs include the recovery sequence and point to lint/export/doctor responsibilities; git diff --check passes.
