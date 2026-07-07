---
created_at: "2026-06-29T20:16:55.721310737+00:00"
id: "atelier-vqhi"
issue_type: "validation"
labels:
- "cli"
- "command-audit"
- "validation"
review:
  kind: pull_request
  number: 50
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-durs
    integration_target: mission/atelier-durs
    merge_strategy: squash
    owner_issue_id: atelier-vqhi
    owner_kind: issue
    review_target: mission/atelier-durs
    work_branch: validation/atelier-vqhi
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T19:10:04.304395363+00:00"
status: "done"
title: "Validate command complexity budget cleanup"
updated_at: "2026-07-06T19:10:04.304395363+00:00"
---

## Description

Validate the command complexity budget cleanup after the review, evidence/history, admin/recovery, and retired-docs epics are implemented. The validator chooses the commands, help output, docs searches, and tests needed to prove the final state from the operator point of view and verifies that the budget treats product/cognitive complexity and architecture/code complexity as separate costs.

## Outcome

Validation records pass, fail, blocked, or deferred status for each linked epic. It includes concrete command/help/docs/test evidence for the surviving review, evidence, history, provider/admin, recovery, and retired-doc behavior; confirms hidden or removed surfaces are not promoted as normal workflow; confirms command-audit guidance applies both product/cognitive and architecture/code complexity; and records baseline checks including cargo fmt -- --check, cargo nextest run, git diff --check, and atelier check.
