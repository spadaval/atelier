---
schema: atelier.review
schema_version: 1
id: atelier-72ob
title: 'The new in-scope rule for clean base merges (crates/atelier-app/src/review_gate.rs:203-227) stales an approval for base planning that leaves the owner''s acceptance and readiness alone. Reproduced: approve, then on the base run atelier issue link V <owner> --role blocked_by, the follow-up validation shape that atelier-1jll recommends. That writes V into the owner''s relationships.blocks. A clean git merge of the base then fails review.current, which atelier-n7g1 accepted before. This fails safe and costs one re-review. Consider allowing additions to an in-scope record''s blocks (work that waits on the owner) in a base merge. The docs (reviews-and-merge.md:68-70) mention only acceptance rewrites, so they should also say that any non-status edit to an in-scope record, relationships included, needs renewed review.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: acffbe36af6bb949038f0313feedb2a5c15affd2
created_at: 2026-10-16T20:09:45.030843122Z
event:
  event: finding
  recommendation: defer
---

The new in-scope rule for clean base merges (crates/atelier-app/src/review_gate.rs:203-227) stales an approval for base planning that leaves the owner's acceptance and readiness alone. Reproduced: approve, then on the base run atelier issue link V <owner> --role blocked_by, the follow-up validation shape that atelier-1jll recommends. That writes V into the owner's relationships.blocks. A clean git merge of the base then fails review.current, which atelier-n7g1 accepted before. This fails safe and costs one re-review. Consider allowing additions to an in-scope record's blocks (work that waits on the owner) in a base merge. The docs (reviews-and-merge.md:68-70) mention only acceptance rewrites, so they should also say that any non-status edit to an in-scope record, relationships included, needs renewed review.
