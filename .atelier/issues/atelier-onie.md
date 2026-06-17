---
created_at: "2026-06-17T18:00:54.109347237+00:00"
id: "atelier-onie"
issue_type: "task"
labels:
- "pr"
- "validator"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement linked_pr_merged validator"
updated_at: "2026-06-17T18:00:54.109347237+00:00"
---

## Description

Implement the `linked_pr_merged` built-in workflow validator.

## Outcome

- The validator reads the issue's `forge_pr` typed field.
- The validator confirms provider/repo/branch consistency where available.
- The validator queries Forgejo and passes only when the linked PR is merged.
- Closed-but-unmerged PRs fail.

## Evidence

- Focused validator tests cover no field, open PR, closed-unmerged PR, merged
  PR, repo mismatch, and branch mismatch.
- Command transcript shows targeted `linked_pr_merged` tests pass.
