---
schema: atelier.review
schema_version: 1
id: atelier-t8bq
title: 'Low. Approval freshness answer changes on the base-merge path (review_gate.rs allowed_base_merge): when the base adds ''X blocked_by Y'' with X in the approved scope and Y outside it, master wrote Y''s blocks, an out-of-scope non-conflicted path, so the approval stayed current after merging the base; now X''s own record changes in relationships, which is not status-only, so review.current fails and the owner needs renewed review. This is stricter, not looser, is the mirror image of S4 (the declarer is written), and docs/product/reviews-and-merge.md states it (''a blocker the record declares ... needs renewed review''). The new merge_gates test covers the branch-side and new-record cases but not this base-merge case. Recording so the accountable agent consciously accepts it against the done-when line ''in-scope record freshness ... same answers''; optionally add a base-merge case to links_declared_by_new_work_keep_approval_and_edits_to_existing_records_stale_it.'
target: atelier-ihss
identity: Independent reviewer (Claude subagent)
commit: af8645378791a3ff66fe5d8ab2a5db7d9cd010bd
created_at: 2026-10-17T06:27:54.485220447Z
event:
  event: finding
  recommendation: no_action
---

Low. Approval freshness answer changes on the base-merge path (review_gate.rs allowed_base_merge): when the base adds 'X blocked_by Y' with X in the approved scope and Y outside it, master wrote Y's blocks, an out-of-scope non-conflicted path, so the approval stayed current after merging the base; now X's own record changes in relationships, which is not status-only, so review.current fails and the owner needs renewed review. This is stricter, not looser, is the mirror image of S4 (the declarer is written), and docs/product/reviews-and-merge.md states it ('a blocker the record declares ... needs renewed review'). The new merge_gates test covers the branch-side and new-record cases but not this base-merge case. Recording so the accountable agent consciously accepts it against the done-when line 'in-scope record freshness ... same answers'; optionally add a base-merge case to links_declared_by_new_work_keep_approval_and_edits_to_existing_records_stale_it.
