---
created_at: "2026-06-14T21:44:36.916839256+00:00"
id: "atelier-kybc"
issue_type: "task"
labels:
- "docs"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-p2ph"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document mission worktree and epic branch operating model"
updated_at: "2026-06-14T21:44:36.916839256+00:00"
---

## Description

Update SPEC, CONTEXT, product docs, and validation guidance so the durable model is Mission = worktree/background workspace, Epic = branch/review boundary, Issue = implementation slice. Outcome: docs no longer teach per-issue worktrees or per-issue review as the default. Evidence: docs diff plus Lint passed. and targeted text search for obsolete guidance.

## Outcome

- SPEC, CONTEXT, product docs, and validation guidance define missions as shared worktrees, epics as review branches, and issues as implementation slices.
- Existing default guidance no longer says each issue or mutating subagent should get its own worktree.
- Existing default guidance no longer requires independent review for every ordinary implementation issue.

## Evidence

- Documentation diff shows the updated mission, epic, issue, branch, worktree, review, and validation language.
- `rg` transcript shows obsolete per-issue worktree and per-issue review guidance has been removed or explicitly classified as exceptional isolation.
- `atelier lint atelier-kybc` passes.
