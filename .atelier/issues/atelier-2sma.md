---
created_at: "2026-06-20T04:17:09.556975939+00:00"
id: "atelier-2sma"
issue_type: "task"
labels:
- "cli"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T04:59:52.620987794+00:00"
status: "done"
title: "Remove special issue close command"
updated_at: "2026-06-20T04:59:52.620987794+00:00"
---

## Description

Imported bundle issue.

## Outcome

- The special `issue close` command is removed.
- Terminal issue closure is performed through configured workflow transitions.
- Help text, docs, tests, and recovery guidance no longer point to `issue close`.

## Evidence

- `atelier issue --help` no longer lists `close`; invoking `atelier issue close` is rejected without a compatibility shim; docs/tests use `atelier issue transition` for terminal closure.
