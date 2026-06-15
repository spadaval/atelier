---
created_at: "2026-06-14T21:45:20.152643432+00:00"
id: "atelier-6nqr"
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
closed_at: "2026-06-14T22:24:23.809423745+00:00"
status: "done"
title: "Update Agent Factory portable workflow guidance"
updated_at: "2026-06-14T22:24:23.809423745+00:00"
---

## Description

Update /root/.agents/skills/agent-factory so portable guidance uses mission workspaces and epic review branches instead of issue worktrees by default. Outcome: SKILL.md, orchestrate, tracker, repo-workflow, and authoring standards describe mission worktree allocation, epic review boundaries, issue-local proof, and justified extra isolation only. Evidence: diff review maps each old per-issue-worktree/review instruction to the new model.

## Outcome

- Agent Factory portable guidance uses mission workspaces and epic review branches as the default coordinated-work model.
- Portable guidance allows extra worktrees only for justified isolation, contention, or parallelism.
- Portable guidance removes automatic independent review requirements from ordinary issues and places review at epic or mission scale.

## Evidence

- Diff of `/root/.agents/skills/agent-factory` files shows old issue-worktree/review guidance replaced by the new model.
- Review artifact maps each changed Agent Factory instruction to Atelier product terminology.
- Targeted `rg` transcript shows no remaining default per-issue worktree instruction in Agent Factory guidance.
