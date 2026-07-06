---
created_at: "2026-06-23T16:21:20.073661392+00:00"
id: "atelier-hl1n"
issue_type: "epic"
labels: []
fields:
  workflow_branch:
    branch_base: mission/atelier-mska
    integration_target: mission/atelier-mska
    merge_strategy: squash
    owner_issue_id: atelier-hl1n
    owner_kind: epic
    review_target: mission/atelier-mska
    work_branch: epic/atelier-hl1n
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ax1g"
  - kind: "issue"
    id: "atelier-muzq"
  - kind: "issue"
    id: "atelier-sdqy"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Unify record-file storage and domain models"
updated_at: "2026-07-06T18:08:39.089571489+00:00"
---

## Description

Refactor storage and domain boundaries so shared record-file mechanics do not become the primary domain abstraction.

## Outcome

- The codebase has unified record-file storage mechanics and concrete domain boundaries for issue, evidence, and review behavior.

## Evidence

- Issue, evidence, and review domain logic operate on concrete domain types.
- Record-file parsing, rendering, discovery, and atomic writes are shared at the storage layer.
