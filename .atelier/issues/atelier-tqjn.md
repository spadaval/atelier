---
created_at: "2026-06-14T16:31:28.200345043+00:00"
id: "atelier-tqjn"
issue_type: "task"
labels:
- "tracker"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add regression coverage for transition closeout retry loops"
updated_at: "2026-06-14T16:31:28.200345043+00:00"
---

## Description

Tests or transcripts should prove --options stays read-only and sequential issue closes do not self-block solely because tracker activity was written.

## Outcome

- Regression coverage captures the postmortem retry-loop failure mode:
  inspection stays read-only, tracker-generated canonical updates do not
  self-block closeout, and non-tracker dirty work still blocks.
- The tests are narrow enough to run during focused workflow validation.

## Evidence

- Test names or validation transcript explicitly cover `--options` read-only
  behavior and tracker-file clean-worktree filtering.
- The test fails against the old behavior or the transcript documents the old
  failure mode before the fix.
- `git diff --check` and `atelier lint` pass.
