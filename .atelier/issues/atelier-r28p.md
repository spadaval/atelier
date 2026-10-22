---
created_at: "2026-10-22T21:58:40.482114676+00:00"
id: "atelier-r28p"
issue_type: "feature"
labels:
- "land"
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-r28p
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-iaas"
  - kind: "issue"
    id: "atelier-ss25"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Keep approvals and land's scope steady through planning records"
updated_at: "2026-10-22T22:12:46.139224870+00:00"
---

## Goal

Planning records written during review do not cost a re-review or a mission-wide re-check. A follow-up filed and linked on an approved branch leaves the approval current (atelier-iaas). A base commit that only adds an issue advancing a mission re-checks no old done claim in that mission (atelier-ss25).

## Done when

- Each child's Done when holds, shown by land and merge tests that check the two commands agree, with independent review.
- Delivers atelier-k8h8 S7's follow-up and planning-commit parts.

## Background

Both children change what the review gate and land's touched-claims scope treat as in scope: `allowed_edge` and approval currency in crates/atelier-app/src/review_gate.rs, and `touched_claims` in crates/atelier-cli/src/commands/session.rs. They build on the shared approval rule from atelier-1ztv (`approval_is_current_to`, `claims.reviewed`, land's approval-validity check) and must keep land and merge agreeing. Since atelier-ihss, links are stored on the declaring record, so linking a new follow-up to its mission writes only the new record.

## Notes

- 2026-10-22 [note] From review of atelier-1ztv (finding atelier-p5qp): docs/product/reviews-and-merge.md:261-264 and the doc comments on approval_accepted and author_identities_to say land judges independence as review.current does. land leaves out authors of every side branch merged into the work branch (original_independent passes HEAD as base), while review.current counts a side branch not on the base. Describe that difference and document the base argument at the call site.
