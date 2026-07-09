---
created_at: "2026-07-06T17:20:25.688108164+00:00"
id: "atelier-vgqe"
issue_type: "task"
labels:
- "cli"
- "docs"
- "product-contract"
fields:
  workflow_branch:
    branch_base: mission/atelier-c0mp
    integration_target: mission/atelier-c0mp
    merge_strategy: squash
    owner_issue_id: atelier-vgqe
    owner_kind: issue
    review_target: mission/atelier-c0mp
    work_branch: task/atelier-vgqe
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4fip"
  - kind: "issue"
    id: "atelier-nzu9"
  - kind: "issue"
    id: "atelier-tdgs"
  - kind: "issue"
    id: "atelier-yf06"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T17:55:18.230078638+00:00"
status: "done"
title: "Define the issue inventory and Mission Overview contract"
updated_at: "2026-07-06T17:55:18.230078638+00:00"
---

## Description

Update the durable CLI and human-output contracts before implementation. Specify `issue list` as flat generic inventory; specify `work missions` as the collapsed cross-mission overview; define default inclusion, `--all` behavior for done missions, mission-to-epic membership, direct and unassigned work treatment, deterministic ordering and boundedness, quiet output, and colorless semantics. Reconcile command-audit text that currently assigns hierarchy to `issue list` or leaves `work queue` as the broad overview.

## Outcome

- Product and output documentation gives `issue list` and `work missions` distinct operator questions and includes representative default and drill-down output shapes.
- The contract settles direct-linked, unassigned, multi-mission, done-mission, ordering, truncation, quiet, TTY color, and `NO_COLOR` behavior so implementation tasks do not invent policy independently.

## Evidence

- Evidence `atelier-x12a` records a passing `git diff --check -- PRODUCT_INTENT.md docs/index.md docs/product` transcript for the contract and guidance changes.
- `docs/product/issue-inventory-and-mission-overview.md` is the durable contract that settles the issue Outcome and downstream implementation policy.
