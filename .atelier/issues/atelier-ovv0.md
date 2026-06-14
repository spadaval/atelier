---
created_at: "2026-06-14T02:52:46.207636953+00:00"
id: "atelier-ovv0"
issue_type: "task"
labels:
- "assignee:root"
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T08:11:10.703840719+00:00"
status: "done"
title: "Make worktree setup and active-work association atomic"
updated_at: "2026-06-14T08:11:10.703840719+00:00"
---

## Description

Ensure worktree setup does not claim or start work until the worktree path and runtime association are valid.

## Outcome

Interrupted or failed worktree setup leaves a clear state that can be retried without manual runtime edits.

## Evidence

Focused tests or validation transcripts cover failed setup and successful retry behavior.
