---
created_at: "2026-08-06T00:14:54.991243693+00:00"
id: "atelier-sfaq"
issue_type: "task"
labels:
- "refactor"
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-sfaq
    owner_kind: issue
    review_target: master
    work_branch: task/atelier-sfaq
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-06T00:25:16.648211701+00:00"
status: "done"
title: "Remove adaptive workflow red tape"
updated_at: "2026-08-06T00:25:16.648211701+00:00"
---

## Description

Remove the process redundancies found in the adaptive-mission red-tape audit
without weakening proof ownership or strategy boundaries.

- Let validation work owned by a parent epic complete from active work with passing evidence, while preserving the stricter reviewed path for standalone validation.
- Remove the duplicate direct-evidence gate from epic completion while retaining complete child proof.
- Express strategy changes through durable adaptation authority rather than a human-commissioned-session concept.
- Mark the superseded Factory Missions notes as historical.
- Update starter policy, product examples, and focused regression coverage.

## Outcome

Adaptive mission delivery preserves independent proof and strategy boundaries without duplicate evidence attachment, validation-state round trips, or hidden conversational authorization.

## Evidence

Focused workflow-policy and CLI tests; repository lint; formatting; whitespace checks.
