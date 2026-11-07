---
schema: atelier.review
schema_version: 2
id: atelier-72ob
title: 'The new in-scope rule for clean base merges (crates/atelier-app/src/review_gate.rs:203-227) stales an approval for base planning that leaves the owner''s acceptance and readiness alone. Reproduced: approve, then on the base run atelier issue link V <owner> --role blocked_by, the follow-up validation shape that atelier-1jll recommends. That writes V into the owner''s relationships.blocks. A clean git merge of the base then fails review.current, which atelier-n7g1 accepted before. This fails safe and costs one re-review. Consider allowing additions to an in-scope record''s blocks (work that waits on the owner) in a base merge. The docs (reviews-and-merge.md:68-70) mention only acceptance rewrites, so they should also say that any non-status edit to an in-scope record, relationships included, needs renewed review.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: acffbe36af6bb949038f0313feedb2a5c15affd2
created_at: 2026-10-16T20:09:45.030843122Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: decb65a8e086ca314995b9534104ce64d67261eb
    recorded_at: 2026-10-16T20:21:13.993866955Z
    resolution_commit: 9c48a595e3a3a3799103820a7a2609c30488fd24
    reason: 'Resolved. docs/product/reviews-and-merge.md now says a record in the owner''s scope may change only in status in either kind of base merge, and that any other base edit, such as a new child, a blocked_by link to the owner (verified: stored in the owner''s blocks) or new acceptance, needs renewed review. This matches allowed_base_merge and allowed_edge.'
    migrated_from: atelier-noax
---

The new in-scope rule for clean base merges (crates/atelier-app/src/review_gate.rs:203-227) stales an approval for base planning that leaves the owner's acceptance and readiness alone. Reproduced: approve, then on the base run atelier issue link V <owner> --role blocked_by, the follow-up validation shape that atelier-1jll recommends. That writes V into the owner's relationships.blocks. A clean git merge of the base then fails review.current, which atelier-n7g1 accepted before. This fails safe and costs one re-review. Consider allowing additions to an in-scope record's blocks (work that waits on the owner) in a base merge. The docs (reviews-and-merge.md:68-70) mention only acceptance rewrites, so they should also say that any non-status edit to an in-scope record, relationships included, needs renewed review.
