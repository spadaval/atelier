---
schema: atelier.review
schema_version: 2
id: atelier-nr0w
title: 'The merge gate still holds superseded work to its open blockers, so land disagrees with transition and the new docs are wrong. crates/atelier-app/src/review_gate.rs:460-465 (blockers.none_open) builds dependency_closure for every scoped id, superseded children included; only completion_claims (:688) skips superseded. Reproduced with the built binary: create Pre, Parent, and Child --parent Parent; link Child blocked_by Pre; start Parent; supersede Child (allowed); close Parent (allowed); land fails with ''atelier-<parent> blockers.none_open: open blockers: <Pre>''. Pre is the superseded child''s own direct blocker. The policy merge_checks validator passes (''no open blockers''); only the hard-coded gate fails. docs/product/work-model.md:93-95 says land does not hold a superseded item to its blockers, which this contradicts. Fix: skip superseded ids when collecting closures at :460, and add a test for a parent branch whose superseded child has a direct open blocker. Or narrow the docs if this is intended. Other checks passed: the new test fails without the fix (blockers.transitive_none_open from the parent''s blocker); hard-coded "superseded" matches workflow.rs:175 and git.rs:343; validators of the transition to superseded (custom blocker validators included) still run on committed claims via the third loop in completion_claims, so custom policies keep their checks; a done item edited to superseded skips only the blocker and evidence checks, which matches what the supersede transition allows.'
target: atelier-lq42
identity: Independent reviewer (Claude subagent)
commit: 57f37fde21611ac5c5f38306b0d0f0b36e5bddc2
created_at: 2026-10-16T19:05:28.464897029Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 0c98c5383f71f05b86b61ed3e11214a62409b82c
    recorded_at: 2026-10-16T19:09:45.451674316Z
    resolution_commit: f83875561add0bfd47cf2e19e15b88198e515e5d
    reason: 'Resolved. evaluate_scope (crates/atelier-app/src/review_gate.rs:463-472) now skips superseded ids when collecting blockers.none_open, so a superseded child''s own and inherited blockers no longer hold the parent''s land/merge; the owner''s own closure is still collected, so blockers the parent itself carries still block. The new test a_superseded_childs_own_blockers_do_not_hold_its_parents_merge fails with the review_gate.rs change reverted (land_invariants_unmet: blockers.none_open: open blockers: <prerequisite>) and passes with it; land and merge both succeed. docs/product/work-model.md now matches the behavior.'
    migrated_from: atelier-olog
---

The merge gate still holds superseded work to its open blockers, so land disagrees with transition and the new docs are wrong. crates/atelier-app/src/review_gate.rs:460-465 (blockers.none_open) builds dependency_closure for every scoped id, superseded children included; only completion_claims (:688) skips superseded. Reproduced with the built binary: create Pre, Parent, and Child --parent Parent; link Child blocked_by Pre; start Parent; supersede Child (allowed); close Parent (allowed); land fails with 'atelier-<parent> blockers.none_open: open blockers: <Pre>'. Pre is the superseded child's own direct blocker. The policy merge_checks validator passes ('no open blockers'); only the hard-coded gate fails. docs/product/work-model.md:93-95 says land does not hold a superseded item to its blockers, which this contradicts. Fix: skip superseded ids when collecting closures at :460, and add a test for a parent branch whose superseded child has a direct open blocker. Or narrow the docs if this is intended. Other checks passed: the new test fails without the fix (blockers.transitive_none_open from the parent's blocker); hard-coded "superseded" matches workflow.rs:175 and git.rs:343; validators of the transition to superseded (custom blocker validators included) still run on committed claims via the third loop in completion_claims, so custom policies keep their checks; a done item edited to superseded skips only the blocker and evidence checks, which matches what the supersede transition allows.
