---
created_at: "2026-06-19T22:42:56.485552874+00:00"
id: "atelier-ih2n"
issue_type: "task"
labels:
- "cli"
- "workflow-policy"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make statuses primary in user-facing output"
updated_at: "2026-06-19T22:42:56.485552874+00:00"
---

## Description

Clean user-facing output so repo-defined statuses are primary and categories are used only for grouping, filtering, or rollup. Remove `category/status` formatting where it makes categories look like lifecycle state.

## Outcome

- User-facing CLI output presents repository-defined statuses as lifecycle state.
- Categories remain available only for grouping, filtering, rollup, and implementation metadata where that distinction is visible.

## Evidence

- CLI integration tests prove issue show/list/status surfaces lead with the configured status name.
- Mission/status CLI integration tests assert rollups still group by high-level category where useful.
