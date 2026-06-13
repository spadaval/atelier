---
created_at: "2026-06-11T02:45:00.803546168+00:00"
id: "atelier-iv68"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kaei"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T03:50:58.325373741+00:00"
status: "done"
title: "Remove --json CLI options for command result rendering"
updated_at: "2026-06-11T03:50:58.325373741+00:00"
---

## Description

Remove command-result --json flags from relevant clap definitions and command dispatch while preserving unrelated diagnostic logging options if approved by the boundary artifact. Acceptance: help output for representative commands no longer lists result --json; invoking removed flags fails with a clear standard CLI error; no command silently ignores the flag.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
