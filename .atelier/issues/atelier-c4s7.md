---
created_at: "2026-06-30T21:34:08.403828655+00:00"
id: "atelier-c4s7"
issue_type: "epic"
labels: []
fields:
  workflow_branch:
    branch_base: mission/atelier-lz6a
    integration_target: mission/atelier-lz6a
    merge_strategy: squash
    owner_issue_id: atelier-c4s7
    owner_kind: epic
    review_target: mission/atelier-lz6a
    work_branch: epic/atelier-c4s7
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lz6a"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "done"
title: "Epic: Mission-isolated publish workflow"
updated_at: "2026-06-30T22:07:16+00:00"
---

## Description

Implement workflow behavior for mission-isolated execution and explicit mission
publish review.

## Outcome

Mission-scoped work no longer needs to check out the configured base branch
after the mission has started.

- Mission workflow includes a terminal publish review state after active work.
- Mission request-publish opens or reuses a review artifact from the mission
  branch to the configured base branch without merging.
- Mission-scoped branch-owning work derives branch base, review target, and
  integration target from the containing mission branch when no nearer owner
  branch applies.
- Status and transition output clearly show mission-derived source and target
  branches.

## Evidence

- Focused integration tests cover mission request-publish, scoped
  validation branch target resolution, and concurrent mission worktrees.
- Existing provider-ordering, review, branch integration, and `git.sync`
  coverage remains green.
- Evidence `atelier-5g4h` records full validation for this epic.
