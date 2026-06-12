---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000m"
issue_type: "epic"
labels:
- "epic"
- "milestone"
- "spec"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000c"
  children:
  - kind: "issue"
    id: "atelier-000f"
  - kind: "issue"
    id: "atelier-000n"
  - kind: "issue"
    id: "atelier-000q"
  - kind: "issue"
    id: "atelier-000v"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Milestone 3: Work, branch, and worktree ergonomics"
updated_at: "2026-06-11T14:41:59.247541613+00:00"
---

## Description

Add Braid/Worktrunk-style work, branch, and Git worktree ergonomics while keeping Git as the underlying mechanism. This milestone creates the normal agent workflow surface around `atelier work start`, `atelier work finish`, `atelier work status`, and `atelier worktree for`, replacing scattered claim/session/timer behavior for normal tracked work.

Scope includes work start/finish, worktree creation, branch helpers, claim/work association, dirty-worktree protection, configured setup hooks, status surfaces, and rebuild in new worktrees. Direct coding-agent process management, live agent-run tracking, retry queues, and session metrics remain deferred.

## Outcome

Work start/finish/status and worktree helper flows exist with accepted command names; work branches and worktrees are associated with records; dirty-main and stale-export safeguards are enforced; new worktrees can rebuild SQLite state; status and setup behavior are driven by explicit workflow/config policy rather than hidden local conventions.

## Evidence

Evidence was not specified in the legacy issue record.
