---
created_at: "2026-06-14T16:31:22.846639192+00:00"
id: "atelier-zrmo"
issue_type: "bug"
labels:
- "assignee:root"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-tqjn"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make issue transition --options read-only"
updated_at: "2026-06-14T16:50:44.569915639+00:00"
---

## Description

Inspecting transition readiness must not write activity entries, mutate records, or dirty the Git worktree.

## Outcome

- `atelier issue transition <id> --options` is a pure read operation.
- Running `--options` does not create activity sidecars, modify issue records,
  refresh canonical timestamps, or otherwise dirty the Git worktree.
- Blocked-transition activity is recorded only when a transition is actually
  attempted.

## Evidence

- Focused CLI test or transcript shows `git status --short` unchanged after
  `issue transition <id> --options`.
- Focused CLI test or transcript shows an actually attempted blocked transition
  still records appropriate durable activity when that is the intended audit
  behavior.
- `git diff --check` and `atelier lint` pass.
