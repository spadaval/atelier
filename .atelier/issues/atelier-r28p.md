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
closed_at: "2026-10-22T22:25:18.348155330+00:00"
status: "done"
title: "Keep approvals and land's scope steady through planning records"
updated_at: "2026-10-22T22:40:12.698976487+00:00"
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
- 2026-10-22 [decision] Resolve finding atelier-p5qp in docs: reviews-and-merge.md, approval_accepted and author_identities_to now say land leaves out authors of every branch merged into the work branch (original_independent passes HEAD as base) while review.current passes the fork point and counts a side branch not on the base; a comment at the call site documents the base argument.
  Reason: 1ztv's decision keeps the asymmetry because merge refusing earlier is the safe direction; only the description was wrong.
- 2026-10-22 [decision] Records outside the scope that did not exist at the approval's pin are planning whether created on the branch or filed on the base after the pin and merged in (finding atelier-g0mh); documented in reviews-and-merge.md Approval freshness and at allowed_edge.
  Reason: Orchestrator decision: such records may be committed straight to the base without review, so letting the branch edit them bypasses no gate; their own done claims still fail claims.reviewed and land.
  Rejected: Exempt only records absent at the pin and at the base sides of walked base merges
- 2026-10-22 [decision] merge runs merge_claim_nodes on the commit it would put on the base under either strategy: the merge commit under merge_commit, the branch head under fast_forward_only, trusting the base head (finding atelier-bu66).
  Reason: Under fast_forward_only the branch head becomes the base head, so it is the tree land on the base will judge; running the same check there restores the 1ztv rule that merge refuses any done claim land would refuse.
  Rejected: File bu66 as a separate follow-up (orchestrator chose FIX NOW)

## Close Reason

### Outcome
Planning records written during review no longer cost a re-review or a mission-wide re-check. A follow-up created on an approved branch can be linked and edited in later commits without staling the approval, because a record counts as new when it did not exist at the approval, while closing it still fails claims.reviewed and land (atelier-iaas). A base commit that adds an issue advancing a mission re-checks no old done claim in that mission; land's touched-claims walk stops at the mission, and doctor still reports old problems (atelier-ss25). The product doc now states the side-branch difference in reviewer independence (finding atelier-p5qp). Delivers atelier-k8h8 S7's follow-up and planning-commit parts.

### Deviations
None.

### Decisions
See the decision notes on atelier-iaas, atelier-ss25 and atelier-r28p.

### Surprises
iaas was already true for a follow-up created and linked in one commit; only a later link or edit staled the approval.

### Open items
`atelier issue link <mission> <issue> --role advances` writes onto the mission's record and still stales an approval; the doc shows the right direction. The claims.reviewed refusal wording is confusing for out-of-scope issues. touched_claims still follows blocked_by in the direction no claim depends on.
