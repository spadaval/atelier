---
created_at: "2026-06-08T19:13:42+00:00"
id: "atelier-000n"
issue_type: "task"
labels:
- "feature"
- "git"
- "milestone"
- "spec"
- "workflow"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-000q"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T14:56:32.539896293+00:00"
status: "done"
title: "Add worktree status and setup ergonomics"
updated_at: "2026-06-10T14:56:32.539896293+00:00"
---

## Description

Add Worktrunk-inspired ergonomics around Atelier worktrees without supervising direct agent runs. Scope includes configured branch/path templates, scan-friendly worktree status, hook-driven setup, optional ignored-cache copying, per-worktree local settings such as dev-server ports, and transparent merge/remove helpers.

Out of scope: launching agents, tracking live agent processes, run retry queues, or session metrics.

## Outcome

Worktree helpers use configured branch/path policy; scan-friendly status JSON shows current worktree, branch, path, dirty state, ahead/behind or base relationship, unpushed commits, associated work, and export freshness where available; setup hooks and ignored-cache copy policy come from workflow config; merge/remove helpers document the underlying Git operations and failure recovery; tests or scripted evidence cover status, setup, and cleanup without launching an agent.

## Evidence

Evidence was not specified in the legacy issue record.
