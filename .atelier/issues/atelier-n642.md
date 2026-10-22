---
created_at: "2026-10-17T19:00:36.339073169+00:00"
id: "atelier-n642"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:06:18.301519475+00:00"
status: "done"
title: "Keep an approval current across a close commit that only touches the item's record"
updated_at: "2026-10-22T21:06:18.301519475+00:00"
---

## Goal

The required order is close, then approve, because `land` on master refuses a done claim made after the last approval (land_base_done_claim). That forced a fourth review round on sdb0 just to approve the close commit, and the memory rule 'approve after close'. A commit that only changes the item's own record (status, Close Reason) adds nothing a reviewer can judge.

## Done when

An approval pinned to commit A stays current for a later commit B when A..B changes only the item's own issue record, by status and Close Reason fields. `land` and `merge` agree on this. Tests cover the close-only commit and a close commit that also edits code (still stale).

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-uw4x (merge refuses what land refuses).

## Unknowns

Whether any other record-only change (notes, links) should be covered too.

## Notes

- 2026-10-22 [decision] "The item's own record" is the approved scope: the owner and its descendants (review_gate::scope), not the owner's parent. A close-only commit for any of them, by status, timestamps and Close Reason, keeps the approval current.
  Reason: Children close on the owner's branch, often after the owner's approval, and merge's review.current already treated the whole scope this way; narrowing it to the owner alone would make children re-trigger review. The parent is outside the approved scope and closes on the base as planning (missions) or on its own branch.
  Rejected: Owner record only; owner plus parent
- 2026-10-22 [decision] Fix the disagreement in land, not merge: land on the base accepts a done claim after an approval pin when review_gate::approval_is_current_to (now pub, the one rule) holds at the claim's commit, with the tip standing in for the base.
  Reason: merge already applied the status/Close Reason rule; land's coverage was the outlier. Passing the tip as the base keeps a clean base merge after approval current, as it was at merge time; using the upstream would wrongly stale merges of unpushed base commits.
  Rejected: Duplicate the status-only check inside git.rs; use the upstream as the base

## Close Reason

An approval pinned at A now stays current for a later close commit that changes only the approved scope's own records (the owner and its children) by status, timestamps and Close Reason, under both `merge` and `land` on the base; a close that also edits code or another field goes stale under both. The rule lives in `review_gate::approval_is_current_to`, now public, and is stated once in docs/product/reviews-and-merge.md (Approval freshness).

### Deviations
merge already applied this rule; only land on the base (`check_base_commits`) disagreed, so the fix is in land, not in the approval-currency check itself.

### Decisions
Scope is owner plus descendants, not the parent; land asks the shared rule with the tip standing in for the base. See this item's decision notes. atelier-uw4x shares the change and closes with the alternative clause of its Done when.

### Surprises
`base_done_claim` returned only the first claim in a commit, so a covered claim could have hidden an uncovered one in the same commit; it now returns all of them.

### Open items
Notes and links after approval still stale it, as before (this item's Unknowns); atelier-iaas takes the follow-up-record case. merge's review.current message still does not say why an approval is stale or name the re-approve command: atelier-cisa. The user memory rule "approve after close" becomes unnecessary once this merges and the CLI is reinstalled.
