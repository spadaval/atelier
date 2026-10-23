---
created_at: "2026-10-17T19:00:36.348998575+00:00"
id: "atelier-i80r"
issue_type: "feature"
labels:
- "land"
- "retro-2026-10-17"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:33:00.474891060+00:00"
status: "done"
title: "Report a pending review from the Stop hook instead of failing land"
updated_at: "2026-10-23T17:33:00.474891060+00:00"
---

## Goal

The Stop hook failed `land` about 8 times across 1w8c, k8h8 and o3lm while a background review was running ('unresolved FIX NOW: resolution needs fresh independent approval', 'no current independent approval'). Each failure was expected and cost a turn explaining so. In the close-then-approve window the hook should say 'awaiting review on X' without blocking.

## Done when

When the only failed land invariants on a work branch are review currency or finding dispositions for the branch's own item, `atelier land --hook claude` returns a non-blocking message naming the item and what review it awaits. Plain `atelier land` still fails. Hook tests cover the pending-review case and a mixed failure that still blocks.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-467y, atelier-ft88, atelier-mp6v.

## Unknowns

How the hook knows a review is in flight versus forgotten; a time bound or a 'review requested' record may be needed.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Decided on atelier-rc8j: no new record; the hook treats a refusal as a pending review when it is on the owner's own work branch and every failed check is review.current or claims.reviewed with no open finding; land still exits non-zero.
- 2026-10-23 [decision] Pending review is applied literally: every failed check must be review.current or claims.reviewed on the owner. A resolved fix_now finding awaiting a fresh approval fails findings.disposed (AwaitingApproval) and so still blocks.
  Reason: The binding atelier-rc8j decision names only those two checks; extending it to findings.disposed is a mission-level call.
  Rejected: Treating findings.disposed with only AwaitingApproval gaps as pending (proposed as a follow-up)

## Close Reason

On the owner's own work branch, when every failed check is the owner's `review.current` (or `claims.reviewed`), the Stop hook returns a systemMessage naming the owner as awaiting an independent review, with land's refusal and its `atelier review request` step; plain land still refuses, and a pending review with another failure or an open finding blocks.

### Deviations
None.

### Decisions
See the decision note: the binding definition is applied literally, so a resolved fix_now finding awaiting a fresh approval (findings.disposed, AwaitingApproval) still blocks.

### Surprises
`claims.reviewed` is produced only by merge, not land, so on land the pending case is review.current alone.

### Open items
Proposed, not filed: decide on atelier-rc8j whether findings.disposed failing only with AwaitingApproval gaps should also count as a pending review; that window was part of the retrospective's turn cost.
