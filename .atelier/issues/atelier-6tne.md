---
created_at: "2026-07-06T20:37:49.336564568+00:00"
id: "atelier-6tne"
issue_type: "epic"
labels:
- "architecture"
- "mission-review"
- "planning"
review:
  kind: pull_request
  number: 66
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-p4z2
    integration_target: mission/atelier-p4z2
    merge_strategy: squash
    owner_issue_id: atelier-6tne
    owner_kind: epic
    review_target: mission/atelier-p4z2
    work_branch: epic/atelier-6tne
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-h3wg"
  - kind: "issue"
    id: "atelier-p2wk"
  children:
  - kind: "issue"
    id: "atelier-0zhd"
  - kind: "issue"
    id: "atelier-2uim"
  - kind: "issue"
    id: "atelier-wlk4"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Define the independent mission-plan review contract"
updated_at: "2026-07-09T16:38:54.554041990+00:00"
---

## Description

Reconcile prior mission-lifecycle decisions with the newly chosen independent review model before implementation begins. This epic owns the product, architecture, domain-language, lifecycle, migration, and authoring-standard contract plus independent validation that the updated contract clearly retains, amends, or supersedes every applicable legacy rule.

## Outcome

- Repository-owned product and architecture sources define mission drafts, independent plan review, exact-revision approval, stale approval, author-reviewer separation, dependency-safe readiness, and migration behavior consistently.
- The repository has one normative, example-backed standard for constructing and reviewing complete mission issue sets.
- The legacy atelier-2uim validation responsibility is migrated into this epic as a compatibility audit of the updated contract, not left ready against the closed atelier-1mga mission or used to validate obsolete direct-readiness behavior.

## Evidence

- The contract child evidence record includes a source-by-source decision matrix covering product intent, domain language, ADRs, workflow policy, validation policy, and the authoring standard for lifecycle state, reviewer independence, graph-revision identity, material edits, dependency readiness, and migration.
- Focused file diff inspection names how atelier-a44d, atelier-ql9k, and atelier-1mga are amended while preserving workflow ownership, Outcome-led planning, validator-derived proof, and evidence-as-receipt boundaries.
- A fresh planner and independent reviewer apply the published standard to one complete graph and the defective graph examples; the resulting classifications and affected issue/dependency paths are recorded on the accountable child issues.
