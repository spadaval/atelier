---
created_at: "2026-06-23T16:21:20.073661392+00:00"
id: "atelier-hl1n"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 48
  provider: forgejo
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
status: "review"
title: "Epic: Unify record-file storage and domain models"
updated_at: "2026-07-06T18:21:24.005227002+00:00"
---

## Description

Refactor storage and domain boundaries so shared record-file mechanics do not become the primary domain abstraction.

## Outcome

- The codebase has unified record-file storage mechanics and concrete domain boundaries for issue, evidence, and review behavior.

## Evidence

- Evidence `atelier-70vn`: typed issue/evidence/review codec and concrete
  record-file service tests pass.
- Evidence `atelier-919w`: concrete app evidence/review services, review-room
  behavior, and workflow validation tests pass.
- Evidence `atelier-aapz`: issue review accessor, parse validation, round-trip,
  and review-room regression tests pass.
- Evidence record `atelier-5x3w`: five focused regression tests pass for
  Forgejo provider accessor persistence/lookup and CLI evidence readiness.
