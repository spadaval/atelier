---
created_at: "2026-06-13T04:01:42.078652500+00:00"
id: "atelier-jxzc"
issue_type: "task"
labels:
- "cli"
- "runtime"
- "worktree"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Make worktree association atomic and repairable"
updated_at: "2026-06-13T04:01:42.078652500+00:00"
---

## Description

Harden issue worktree setup and active-work association so failed runtime setup cannot partially claim or start unrelated work. Provide a clear repair path for stale active-work association after interrupted or failed subagent runs.

## Outcome

- `atelier worktree for <issue-id>` does not claim, start, or otherwise mutate
  lifecycle state until the worktree and runtime active-work association are
  successfully established.
- Failed worktree setup reports the failed step and leaves no misleading active
  work association behind.
- A first-class repair path exists for clearing or reconciling stale active-work
  association after interrupted or failed subagent runs.
- Agent Factory guidance requires isolated issue worktrees for mutating
  subagents unless the assignment explicitly explains why shared checkout work
  is safer.

## Evidence

- Focused test or transcript simulates failed runtime association and proves no
  unrelated issue is claimed or started.
- Transcript proves the repair command or documented recovery path clears stale
  active-work state without editing canonical records by hand.
- File-change review shows Agent Factory worktree/subagent guidance names the
  isolation rule.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and relevant
  worktree/runtime tests pass.
