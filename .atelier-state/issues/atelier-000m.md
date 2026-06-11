---
acceptance: []
blocks:
- "atelier-000c"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-0009"
- "atelier-001a"
- "atelier-001f"
evidence_required: []
id: "atelier-000m"
issue_type: "epic"
labels:
- "epic"
- "milestone"
- "spec"
- "worktree"
links: []
parent: null
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Milestone 3: Work, branch, and worktree ergonomics"
updated_at: "2026-06-11T14:41:59.247541613+00:00"
---

Add Braid/Worktrunk-style work, branch, and Git worktree ergonomics while keeping Git as the underlying mechanism. This milestone creates the normal agent workflow surface around `atelier work start`, `atelier work finish`, `atelier work status`, and `atelier worktree for`, replacing scattered claim/session/timer behavior for normal tracked work.

Scope includes work start/finish, worktree creation, branch helpers, claim/work association, dirty-worktree protection, configured setup hooks, status surfaces, and rebuild in new worktrees. Direct coding-agent process management, live agent-run tracking, retry queues, and session metrics remain deferred.

## Acceptance

Work start/finish/status and worktree helper flows exist with accepted command names; work branches and worktrees are associated with records; dirty-main and stale-export safeguards are enforced; new worktrees can rebuild SQLite state; status and setup behavior are driven by explicit workflow/config policy rather than hidden local conventions.
