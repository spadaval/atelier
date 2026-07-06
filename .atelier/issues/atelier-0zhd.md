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
status: "todo"
title: "Adopt the independent mission-plan review architecture"
updated_at: "2026-07-06T20:37:49.367809982+00:00"
---

## Description

Assigned subskill: plan. Update the durable product, domain, architecture, ADR, workflow, and validation contracts before dependent implementation. Explicitly reconcile the earlier simple-lifecycle and Outcome-only readiness decisions, distinguish mission-plan review from code review and outcome validation, define reviewer independence and material authorship, specify exact graph revision identity and approval invalidation, define the draft to plan-review to ready lifecycle, and choose an explicit migration rule for existing draft and ready missions. Review events may reuse native findings, change-request, resolution, approval, and freshness infrastructure, but mission-plan approval is not merge authority.

## Outcome

- Durable repository sources consistently specify `draft -> plan_review -> ready -> in_progress` mission planning and execution semantics.
- Only an independent reviewer can approve a mission plan, approval names the exact reviewed mission graph revision, and material graph edits invalidate it.
- The contract explicitly supersedes or amends incompatible decisions recorded by atelier-a44d, atelier-ql9k, and atelier-1mga rather than leaving contradictory guidance live.
- Existing draft and ready missions have an explicit, non-silent migration or grandfathering rule.

## Evidence

Evidence was not specified in the bundle.
