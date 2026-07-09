---
created_at: "2026-07-06T17:20:25.688932425+00:00"
id: "atelier-nzu9"
issue_type: "epic"
labels:
- "cli"
- "inventory"
review:
  kind: pull_request
  number: 60
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-c0mp
    integration_target: mission/atelier-c0mp
    merge_strategy: squash
    owner_issue_id: atelier-nzu9
    owner_kind: epic
    review_target: mission/atelier-c0mp
    work_branch: epic/atelier-nzu9
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g5fl"
  children:
  - kind: "issue"
    id: "atelier-kiyq"
  - kind: "issue"
    id: "atelier-yf06"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Epic: Rework issue list as simple inventory"
updated_at: "2026-07-09T16:24:21.636615252+00:00"
---

## Description

Replace the current grouped queue-style implementation behind `atelier issue list` with a cohesive generic inventory path. This epic owns the flat read model, rendering, filtering, help, and compatibility removal for the issue-list surface.

## Outcome

- `atelier issue list` is a simple flat inventory whose rows, filters, ordering, summary, empty state, and quiet IDs are implemented and tested independently from work dashboards and hierarchy traversal.
- Operational ready and blocked selection routes to `atelier work ready` and `atelier work blocked`; `issue list` does not preserve aliases or hidden grouping behavior that contradicts the approved contract.

## Evidence

Evidence was not specified in the bundle.
