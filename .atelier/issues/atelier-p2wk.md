---
created_at: "2026-07-06T20:37:49.331991658+00:00"
id: "atelier-p2wk"
issue_type: "epic"
labels:
- "mission-review"
- "workflow"
review:
  kind: pull_request
  number: 73
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-p4z2
    integration_target: mission/atelier-p4z2
    merge_strategy: squash
    owner_issue_id: atelier-p2wk
    owner_kind: epic
    review_target: mission/atelier-p4z2
    work_branch: epic/atelier-p2wk
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t876"
  children:
  - kind: "issue"
    id: "atelier-3v2d"
  - kind: "issue"
    id: "atelier-72k4"
  - kind: "issue"
    id: "atelier-amfw"
  - kind: "issue"
    id: "atelier-fqzt"
  - kind: "issue"
    id: "atelier-wyxn"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Enforce reviewed and dependency-safe mission readiness"
updated_at: "2026-07-16T22:10:09.548523296+00:00"
---

## Description

Implement the repository-owned records, workflow transitions, validators, diagnostics, and migration needed to make independent plan review and declared dependency closure unavoidable at execution time.

## Outcome

- Mission readiness and start transitions enforce independent, fresh approval of the current plan graph and reject open declared blockers.
- Issue execution enforces declared dependencies, normal operator surfaces explain failures, and existing mission readiness state is handled according to the approved migration contract.

## Evidence

- Child evidence provides focused positive and negative transcripts for exact-revision approval, stale approval, reviewer independence, direct and transitive blocker closure, issue dependency readiness, migration, rebuild, and normal recovery output.
- The epic review artifact maps each changed workflow/record/CLI boundary to its accountable child and confirms that direct record edits and alternate transition paths cannot bypass the same policy.
- atelier-t876 independently reruns the cross-child scenario matrix, including exclusion of draft mission work from ready surfaces, and attaches classified first-class evidence IDs before this epic is treated as complete.
