---
created_at: "2026-06-12T00:59:11.815914277+00:00"
id: "atelier-o3w3"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T03:25:29.887215590+00:00"
status: "done"
title: "Specify issue start and work command relationship"
updated_at: "2026-06-12T03:25:29.887215590+00:00"
---

## Description

Decide how the intuitive start surface relates to existing work-start and worktree behavior. Direction for this mission: `atelier start <issue-id>` should be the normal entrypoint. It marks/associates the issue as in progress, runs fast start gate checks, and follows configurable workspace policy. The default policy should be low-friction for prototypes and single-person projects: allow working directly in the current checkout/main branch without forcing branch or worktree setup. Projects can opt into stricter branch or worktree modes when they need review workflows, parallelism, isolation for untracked support files, or cross-system collaboration. Explicit worktree commands remain advanced/orchestrator-facing for preparing, inspecting, merging, or removing Git worktrees outside the normal start flow.
final command shapes, fast gate-check behavior, branch/worktree policy, help text, and Agent Factory guidance updates are specified. The design defines a progressive workspace policy model such as current-checkout, branch, or worktree modes; main-branch behavior such as allow/warn/block; branch/path templates; and per-command override behavior if any. The design names obsolete start surfaces for removal rather than compatibility aliases, and ensures users are not required to understand sessions, branches, or worktrees before starting ordinary work.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
