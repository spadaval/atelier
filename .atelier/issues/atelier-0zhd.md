---
created_at: "2026-07-06T20:37:49.367809982+00:00"
id: "atelier-0zhd"
issue_type: "task"
labels:
- "adr"
- "architecture"
- "mission-review"
- "subskill-plan"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-wlk4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-09T16:21:09.823456030+00:00"
status: "done"
title: "Adopt the independent mission-plan review architecture"
updated_at: "2026-07-09T16:21:09.823456030+00:00"
---

## Description

Assigned subskill: plan. Update the durable product, domain, architecture, ADR, workflow, and validation contracts before dependent implementation. Explicitly reconcile the earlier simple-lifecycle and Outcome-only readiness decisions, distinguish mission-plan review from code review and outcome validation, define reviewer independence and material authorship, specify exact graph revision identity and approval invalidation, define the configured draft to plan-review to ready lifecycle, and choose an explicit migration rule for existing draft and ready missions. Preserve atelier-1mga's Outcome-led planning and validator-derived proof boundary: review expands issue-set readiness judgment but does not require planners to prewrite validation transcripts or evidence receipts. Preserve workflow ownership and avoid a general arbitrary-workflow engine beyond the configured mission contract. Migrate the open legacy validator atelier-2uim into the contract epic to audit those retained boundaries and the explicit supersession, rather than leaving it ready to validate the obsolete direct-readiness lifecycle against a missing mission branch. Review events may reuse native findings, change-request, resolution, approval, and freshness infrastructure, but mission-plan approval is not merge authority.

## Outcome

- Durable repository sources consistently specify configured `draft -> plan_review -> ready -> in_progress` mission planning and execution semantics without turning code-review merge state into mission readiness.
- Only an independent reviewer can approve a mission plan, approval names the exact reviewed mission graph revision, and material graph edits invalidate it.
- The contract explicitly supersedes or amends incompatible decisions recorded by atelier-a44d, atelier-ql9k, and atelier-1mga rather than leaving contradictory guidance live.
- The contract explicitly migrates atelier-2uim from obsolete lifecycle validation to an independent compatibility audit sequenced after the contract and authoring-standard updates and before dependent implementation.
- Existing draft and ready missions have an explicit, non-silent migration or grandfathering rule.

## Evidence

- The artifact diff explicitly cites atelier-a44d, atelier-ql9k, and atelier-1mga and identifies each retained rule, amended rule, and superseded rule; searches of live product, domain, architecture, ADR, workflow, and validation sources find no contradictory lifecycle or approval statement.
- `atelier issue show atelier-2uim`, `atelier issue transition atelier-2uim`, and `atelier work ready` transcripts show the migrated validator belongs to the p4z2 contract epic, waits on the authoring standard, no longer requires missing `mission/atelier-1mga`, and is not advertised to validate obsolete behavior.
- A contract table fixes the canonical graph-revision inputs, material versus non-material edits, author/material-editor identity, independent-review rule, finding resolution and approval freshness, migration behavior for draft/ready/active/terminal missions, and the boundary from code review, outcome validation, and merge authority.
- The contract requires a CLI transcript for the observable regression that the current `draft -> ready` transition and ready-work projection must stop presenting a draft mission or its descendants as executable before current independent approval, while retaining Outcome-led plans and validator-authored execution proof.
- The accountable issue carries focused documentation/ADR diff evidence plus `atelier check atelier-0zhd`; the independent validator assigned atelier-t876 verifies the contract through public command scenarios rather than accepting the documents alone.
