---
created_at: "2026-06-15T05:22:48.695501224+00:00"
id: "atelier-c0f1"
issue_type: "task"
labels:
- "cleanup"
- "cli"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T05:55:40.733526673+00:00"
status: "done"
title: "Remove legacy session current-work creation surface"
updated_at: "2026-06-15T05:55:40.733526673+00:00"
---

## Description

Remove the legacy session/current-work creation surface from issue creation. New work can still be created, but it should not silently become current work through runtime-only session state.

## Outcome

- `atelier issue create --work` is removed because it exposes the legacy session/current-work model after public `session` commands have already been removed.
- `atelier issue create --help` no longer describes current-session or active-work behavior.
- Creating an issue does not write runtime session or work_association rows that affect `atelier status`.

## Evidence

- Help transcript proves `atelier issue create --help` no longer exposes `--work` or current-session wording.
- Command transcript proves creating an issue does not change the status-derived current-work set unless the issue status changes through normal workflow.
- Focused CLI tests for issue creation and rejected `--work` behavior pass.
- `atelier lint atelier-c0f1` and `atelier export --check` pass.
