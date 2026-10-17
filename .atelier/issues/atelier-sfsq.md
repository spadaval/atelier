---
created_at: "2026-10-17T17:25:59.016985931+00:00"
id: "atelier-sfsq"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-sfsq
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Close work items with an implementer report in Close Reason"
updated_at: "2026-10-17T17:57:36.918241843+00:00"
---

## Goal

S2 of docs/strategies/review-and-introspection.md (mission atelier-r6bt). Close Reason holds only the outcome. What an implementer deviated on, decided and was surprised by, and what they left open, is lost. The implementer report extends Close Reason so it can be written after approval without making that approval stale, and so `debrief` (S4) can show it.

## Done when

- A workflow transition can require report parts in addition to `close_reason`: `required_fields` accepts `report.deviations`, `report.decisions`, `report.surprises` and `report.open_items`, and only on done-category transitions. Each part is a `### Deviations`, `### Decisions`, `### Surprises` or `### Open items` subsection of Close Reason. The text before the first subsection is the required Outcome.
- The close transition reads Close Reason from the record body when it is already written, or from `--reason-file <path>`; `--reason` still sets the Outcome. A missing or empty required part fails the transition, naming the part and the recovery step (edit the record's Close Reason, or pass `--reason-file`).
- `land` checks the required parts on committed claims, as it checks Close Reason today. Done records committed before the policy required a part stay valid.
- This repository's policy requires all four parts on the work `close` transition (supersede stays outcome-only). Writing the report after approval does not make the approval stale.
- Transition and `land` tests cover each point. work-model.md, workflow-configuration.md and the Agent Factory skill's handoff and tracker guidance describe the report.

## Context

Close Reason's content is defined in docs/product/work-model.md (atelier-43m1). review_gate.rs already exempts Close Reason from approval freshness (`body_without_close_reason`), and records allow `###` subheadings inside sections. land's required-field gate is in review_gate.rs, and the transition check is `required_field_failures` in crates/atelier-cli/src/commands/workflow.rs.

## Unknowns

How land tells a claim made before the policy required a part from a new one. The claim's commit and the base policy at that commit are both available.

## Notes

- 2026-10-17 [decision one-way] Store the report as ### Deviations, ### Decisions, ### Surprises and ### Open items subsections of Close Reason
  Reason: Close Reason is already exempt from approval freshness and checked by land, so the report inherits both without new record shape
  Rejected: a separate Report section
  Rejected: typed report notes
- 2026-10-17 [decision] Check report parts in land only on claims new to the branch, not on claims the base already holds
  Reason: Done records from before the policy required the report must stay valid, and claim_nodes already separates retained claims from new ones
  Rejected: store the policy version on each claim
- 2026-10-17 [decision] With no flag, the close transition uses the Close Reason already in the record body; --reason replaces only the Outcome
  Reason: Strategy S2 says input comes from the record body or a file, not a flag, and keeping parts lets an agent fix a one-line Outcome without rewriting the report
- 2026-10-17 [decision] Report all missing parts as one --reason-file blocker
  Reason: IssueTransitionOption counts one blocker per missing flag, and a report file also carries the Outcome
