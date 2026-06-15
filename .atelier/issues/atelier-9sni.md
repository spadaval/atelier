---
created_at: "2026-06-14T21:43:56.083152947+00:00"
id: "atelier-9sni"
issue_type: "epic"
labels:
- "agent-factory"
- "docs"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1cib"
  - kind: "issue"
    id: "atelier-6nqr"
  - kind: "issue"
    id: "atelier-8gum"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T04:31:15.002349990+00:00"
status: "done"
title: "Epic: Update Agent Factory for mission workspaces and epic review"
updated_at: "2026-06-15T04:31:15.002349990+00:00"
---

## Description

Update portable Agent Factory instructions and this repo binding so orchestrators allocate mission workspaces, use epic branches as review boundaries, and stop defaulting to isolated issue worktrees. Outcome: subagent prompts name mission worktree, parent epic branch, proof destination, and parallelism/isolation rules. Evidence: Agent Factory diff, repo binding diff, and docs review prove the guidance is consistent with Atelier product docs.

## Outcome

- Agent Factory portable guidance and this repo binding use mission workspaces and epic branches by default.
- Subagent assignment templates name mission worktree, parent epic branch, issue slice, proof destination, and isolation rules.
- Ordinary issue assignments no longer require independent review unless risk or parent scope demands it.

## Evidence

- Child issue proof from atelier-6nqr, atelier-8gum, and atelier-1cib maps to portable skill, repo binding, and template changes.
- Review artifact compares Agent Factory guidance against Atelier product docs.
- Targeted `rg` transcript shows obsolete per-issue worktree guidance is removed from Agent Factory instructions.
