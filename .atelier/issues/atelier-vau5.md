---
created_at: "2026-06-14T05:58:48.026698534+00:00"
id: "atelier-vau5"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:58:31.793989798+00:00"
status: "done"
title: "Remove Claude integrations command surface"
updated_at: "2026-06-14T07:58:31.793989798+00:00"
---

## Description

Remove the integrations command group and Claude Code integration support from help, docs, dispatch, and tests.

## Outcome

The `integrations` command group and Claude Code integration support are
removed from root help, command dispatch, docs, bundled resources, and tests.
Atelier does not advertise or support Claude Code hooks as a product feature.

## Evidence

- `atelier --help` no longer lists `integrations`.
- `atelier integrations ...` fails as an unknown or removed command with
  corrective text that does not advertise Claude support.
- Source and docs search shows Claude integration command code/resources/docs
  removed or explicitly historical.
- Focused CLI tests cover root help and rejected integrations command.
- `git diff --check`, `atelier lint`, and focused command-surface tests pass.
