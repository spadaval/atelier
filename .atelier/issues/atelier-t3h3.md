---
created_at: "2026-06-19T19:38:47.344978694+00:00"
id: "atelier-t3h3"
issue_type: "epic"
labels:
- "implementation"
- "prune"
review:
  kind: pull_request
  number: 63
  provider: forgejo
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-t3h3
    owner_kind: epic
    review_target: master
    work_branch: epic/atelier-t3h3
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-f55w"
  - kind: "issue"
    id: "atelier-iq7f"
  - kind: "issue"
    id: "atelier-rgpl"
  - kind: "issue"
    id: "atelier-w1z8"
  - kind: "issue"
    id: "atelier-x3dy"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Prune command implementation"
updated_at: "2026-07-09T16:34:59.929272278+00:00"
---

## Description

Implement the prune command surface after the retention contract lands. The
implementation must reduce repository and local workspace accumulation while
preserving proof, active work, and unmerged changes by default.

## Outcome

- Operators can inspect prune candidates before applying changes.
- Local ignored runtime/cache/diagnostic cleanup, Git branch/worktree cleanup,
  and canonical record archive/compaction each follow the documented safety
  contract.
- Destructive apply paths require explicit confirmation or force and report what
  changed.
- Protected candidates explain why they were skipped.

## Evidence

- Focused CLI tests cover dry-run output, destructive apply confirmation,
  active/open work protection, required evidence protection, dirty worktree
  protection, and unmerged branch protection.
- Command transcript records focused prune tests, `atelier lint`, and
  `git diff --check`.
- Validation issue `atelier-iq7f` attaches independent closeout proof.
