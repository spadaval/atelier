---
created_at: "2026-06-29T20:15:57.089955580+00:00"
id: "atelier-411x"
issue_type: "epic"
labels:
- "cli"
- "complexity"
- "docs"
fields:
  workflow_branch:
    branch_base: mission/atelier-durs
    integration_target: mission/atelier-durs
    merge_strategy: squash
    owner_issue_id: atelier-411x
    owner_kind: epic
    review_target: mission/atelier-durs
    work_branch: epic/atelier-411x
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-6knt"
  - kind: "issue"
    id: "atelier-fmb7"
  - kind: "issue"
    id: "atelier-p0am"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Collapse retired command audit and stale guidance"
updated_at: "2026-07-01T05:58:40.233517218+00:00"
---

## Description

The command audit still preserves old command names, retired behavior, and stale examples in places that should be compact historical references. Retired surfaces should not read like supported manuals.

## Outcome

Retired command pages are collapsed to tombstones or short history notes, stale role-guide and product-doc references are removed or rewritten to current commands, and command-audit documentation presents the current complexity-budget decisions instead of preserving old workflow instructions.
