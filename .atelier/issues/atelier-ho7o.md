---
created_at: "2026-10-17T17:25:59.150801823+00:00"
id: "atelier-ho7o"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ho7o
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-stsa"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Parse mission criteria and let evidence receipts name the criteria they judge"
updated_at: "2026-10-17T18:04:48.912369448+00:00"
---

## Goal

Implement decision atelier-et6q so `work mission` (atelier-stsa, S8) and `debrief` (S4) can show each success criterion with its proof. Criteria are parsed from the strategy, and evidence receipts name the criteria they judge, each with its own result.

## Done when

- One parser reads a mission's criteria: `- <slug>: <text>` bullets under the Success Criteria heading of the linked strategy, or under the mission's own Success criteria section when it has no strategy. Slugs are lowercase kebab-case or `S<n>` and unique within the mission, kept in document order. Continuation lines belong to the bullet before them.
- `atelier evidence record --criterion <slug>=<result>` (repeatable) stores per-criterion results on the receipt, and `evidence show` prints them.
- `check` refuses a receipt that names a criterion its target's mission does not define (the target is the mission or an item advancing it), and one that names a criterion with no mission. The refusal names the slug and the receipt.
- A proof lookup returns, for each criterion, the receipts naming it with their results and freshness.
- Unit tests cover the parser (this repository's strategies parse without error), and CLI tests cover recording, showing and the `check` refusals. validation.md and mission-authoring.md describe criterion naming.

## Context

The decision record atelier-et6q holds the rules. Evidence record shape: docs/product/validation.md and crates/atelier-records. Freshness: crates/atelier-app/src/evidence_freshness.rs.
