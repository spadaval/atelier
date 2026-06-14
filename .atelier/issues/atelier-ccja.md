---
created_at: "2026-06-14T21:45:09.697720927+00:00"
id: "atelier-ccja"
issue_type: "task"
labels:
- "cleanup"
- "worktree"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove issue-worktree default behavior from public workflow"
updated_at: "2026-06-14T21:45:09.697720927+00:00"
---

## Description

Remove or repurpose the current issue-worktree default so ordinary issue work does not create .atelier-worktrees/<issue-id>. Outcome: help, product docs, and command routing no longer present issue worktrees as the default; explicit isolation remains possible only when justified by policy. Evidence: help transcript, tests, and residue search for obsolete per-issue worktree guidance.

## Outcome

- Public help and product docs no longer present `atelier worktree for <issue-id>` as normal issue workflow.
- Any remaining issue-level workspace isolation is explicitly exceptional and requires justification.
- Tests or residue searches prevent reintroducing per-issue worktree guidance.

## Evidence

- Help transcript proves issue-worktree default guidance is absent or replaced.
- `rg` transcript proves obsolete per-issue worktree wording is removed from docs and Agent Factory binding.
- Focused tests prove normal issue start/finish does not create an issue worktree.
