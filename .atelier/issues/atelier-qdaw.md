---
created_at: "2026-06-14T05:58:41.157619088+00:00"
id: "atelier-qdaw"
issue_type: "task"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add doctor fix for local derived-state repair"
updated_at: "2026-06-14T05:58:41.157619088+00:00"
---

## Description

Add doctor --fix to repair ignored runtime/cache/projection state without editing tracked .atelier canonical records.

## Outcome

`atelier doctor --fix` repairs ignored local runtime/cache/projection state
when safe and refuses to edit tracked `.atelier/` canonical records. Normal
operators use doctor repair instead of proactively running export or rebuild.

## Evidence

- `atelier doctor --help` documents `--fix` and its boundary.
- Focused tests cover repairing missing or stale ignored runtime/projection
  state.
- Negative test proves `doctor --fix` does not modify tracked `.atelier`
  canonical record files when canonical Markdown is malformed.
- Root help or docs no longer present `export`/`rebuild` as normal operator
  repair commands.
- `git diff --check`, `atelier lint`, and focused doctor tests pass.
