---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000v"
issue_type: "task"
labels:
- "feature"
- "git"
- "spec"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000n"
  - kind: "issue"
    id: "atelier-000q"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement worktree and branch helper commands"
updated_at: "2026-06-10T14:51:59.743735264+00:00"
---

Add helpers for normal work branches, optional mission branches, and work-associated worktrees while remaining a convenience layer over Git. Helpers should consume the configured branch/path policy and workflow setup hooks rather than launching or supervising agents.

## Acceptance

`atelier worktree for <id>` creates or locates the configured worktree; branch naming follows policy; implementation warns or fails on main according to policy; new worktrees rebuild SQLite from `.atelier-state`; setup hooks can prepare ignored caches or per-worktree local settings; tests or scripted evidence cover branch/worktree association and merge-helper behavior without launching an agent.
