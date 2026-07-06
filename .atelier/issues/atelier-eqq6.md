---
created_at: "2026-06-29T20:13:43.814775362+00:00"
id: "atelier-eqq6"
issue_type: "epic"
labels:
- "admin"
- "cli"
- "complexity"
review:
  kind: pull_request
  number: 44
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-durs
    integration_target: mission/atelier-durs
    merge_strategy: squash
    owner_issue_id: atelier-eqq6
    owner_kind: epic
    review_target: mission/atelier-durs
    work_branch: epic/atelier-eqq6
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-g87o"
  - kind: "issue"
    id: "atelier-ie31"
  - kind: "issue"
    id: "atelier-mxnv"
  - kind: "issue"
    id: "atelier-vqdm"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T18:20:05.346194494+00:00"
status: "done"
title: "Epic: Hide or remove provider and recovery escape hatches"
updated_at: "2026-07-06T18:20:05.346194494+00:00"
---

## Description

Provider setup, branch recovery, destructive maintenance, and raw diagnostics are valuable only when explicitly routed by setup or recovery. They should not appear as normal workflow or role-guide paths.

## Outcome

Forgejo/provider setup, branch recovery, maintenance delete, and hidden diagnostic commands have explicit Hide, Fold, Keep, or Remove decisions. Root help, role guides, command audit, and tests agree that these surfaces are not normal workflow. Any surviving recovery command is reachable only from explicit setup, failed transition, check, or admin recovery guidance.
