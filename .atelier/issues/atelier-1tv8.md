---
created_at: "2026-06-14T02:52:18.434908462+00:00"
id: "atelier-1tv8"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Require validation checkout ownership in handoffs"
updated_at: "2026-06-14T02:52:18.434908462+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, add a portable
handoff field or instruction that states whether validation ran from the root
checkout or an issue worktree, and which checkout owns follow-up validation.

## Outcome

Subagent handoffs no longer leave the orchestrator guessing which worktree holds runtime state or proof.

## Evidence

- File diff in `AGENTFACTORY.md` or the Agent Factory prompt template shows checkout ownership and dirty-state expectations.
- The wording stays within Agent Factory's portable coordination role and
  routes repository-specific worktree commands to Atelier-owned surfaces.
- `git diff --check` passes for the documentation change.
