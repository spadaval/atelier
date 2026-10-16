---
schema: atelier.review
schema_version: 1
id: atelier-olog
title: 'Resolved. evaluate_scope (crates/atelier-app/src/review_gate.rs:463-472) now skips superseded ids when collecting blockers.none_open, so a superseded child''s own and inherited blockers no longer hold the parent''s land/merge; the owner''s own closure is still collected, so blockers the parent itself carries still block. The new test a_superseded_childs_own_blockers_do_not_hold_its_parents_merge fails with the review_gate.rs change reverted (land_invariants_unmet: blockers.none_open: open blockers: <prerequisite>) and passes with it; land and merge both succeed. docs/product/work-model.md now matches the behavior.'
target: atelier-lq42
identity: Independent reviewer (Claude subagent)
commit: 0c98c5383f71f05b86b61ed3e11214a62409b82c
created_at: 2026-10-16T19:09:45.451674316Z
event:
  event: disposition
  finding: atelier-nr0w
  decision: fix_now
  supersedes: null
  resolution_commit: f83875561add0bfd47cf2e19e15b88198e515e5d
  follow_up: null
---

Resolved. evaluate_scope (crates/atelier-app/src/review_gate.rs:463-472) now skips superseded ids when collecting blockers.none_open, so a superseded child's own and inherited blockers no longer hold the parent's land/merge; the owner's own closure is still collected, so blockers the parent itself carries still block. The new test a_superseded_childs_own_blockers_do_not_hold_its_parents_merge fails with the review_gate.rs change reverted (land_invariants_unmet: blockers.none_open: open blockers: <prerequisite>) and passes with it; land and merge both succeed. docs/product/work-model.md now matches the behavior.
