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
closed_at: "2026-06-14T08:28:48.625436664+00:00"
status: "done"
title: "Document canonical projection recovery model"
updated_at: "2026-06-14T08:28:48.625436664+00:00"
---

## Description

Document the difference between canonical Markdown writes and projection refresh
after the stale-state and recovery behavior is stable, plus the normal repair
order for stale projections, invalid canonical Markdown, and runtime/cache
artifacts.

## Outcome

Agents can tell whether a durable write landed and what to do when projection freshness blocks reads or workflow commands.

## Evidence

- Docs include the recovery sequence and point to the implemented lint, doctor,
  and any remaining advanced rebuild/export responsibilities.
- File diff or review artifact shows examples distinguishing durable canonical
  writes from rebuildable local projection/runtime state.
- `git diff --check` and `atelier lint` pass.
