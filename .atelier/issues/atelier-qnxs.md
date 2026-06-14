---
created_at: "2026-06-14T05:58:17.057323117+00:00"
id: "atelier-qnxs"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add explicit mission close command"
updated_at: "2026-06-14T05:58:17.057323117+00:00"
---

## Description

Add mission close as the normal mission closeout command and stop routing ordinary mission closeout through raw mission status update.

## Outcome

`atelier mission close <id> --reason "..."` is the normal mission closeout
command and runs mission closeout gates. Ordinary operators do not close
missions through raw `mission update --status closed`.

## Evidence

- `atelier mission --help` shows `close` with a close reason.
- Focused tests cover successful mission close after gates pass and rejection
  when linked work, proof, audit, health, or worktree gates fail.
- Help/docs transcript shows `mission update --status closed` is not the
  ordinary closeout path.
- `atelier mission status` next actions point to `mission close` when closeout
  is ready.
- `git diff --check`, `atelier lint`, and focused mission close tests pass.
