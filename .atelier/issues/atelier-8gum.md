---
created_at: "2026-06-14T21:45:22.040859165+00:00"
id: "atelier-8gum"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update Atelier repo Agent Factory binding"
updated_at: "2026-06-14T21:45:22.040859165+00:00"
---

## Description

Update AGENTFACTORY.md and related repo guidance so this repository routes orchestrators to mission worktrees and epic branches. Outcome: subagent handoff requirements name mission worktree, parent epic branch, proof destination, validation checkout, and branch/worktree cleanup owner. Evidence: docs diff, targeted search for obsolete mutating-subagent issue-worktree defaults, and focused lint proof show the repo binding matches the new operating model.

## Outcome

- `AGENTFACTORY.md` routes this repository's orchestrators to mission worktrees and epic branches.
- Subagent handoff guidance names mission worktree, parent epic branch, proof destination, validation checkout, and cleanup owner.
- Repo binding no longer says mutating subagents should normally use isolated issue worktrees.

## Evidence

- `AGENTFACTORY.md` diff shows mission workspace and epic branch guidance.
- Targeted `rg` transcript proves old mutating-subagent issue-worktree default is gone from repo guidance.
- `atelier lint atelier-8gum` passes.
