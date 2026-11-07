---
schema: atelier.review
schema_version: 2
id: atelier-jbcj
title: finding on atelier-jzh9 @963c75a7
target: atelier-jzh9
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:32.613223179Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner/ll3p (claude-opus-5-5)
    commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    recorded_at: 2026-10-31T00:11:33.162720610Z
    resolution_commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    reason: Fixed in the planning revision; g2h9 is settled by the re-review's approval of atelier-80qd
    migrated_from: atelier-vk3f
---

R5: Scope leaves two consequences of the comparison open. (a) "same warning" would print a false message: review.rs:148, review_gate.rs:689, review.rs:574 and merge_sync.rs:401 all say the identity matches a Git author. (b) land: original_independent (review_gate.rs:1858) calls author_identities_to, not the named author_identities, so changing the wrapper leaves land and approval_accepted accepting the approval while merge refuses, and changing the inner function changes land, reading the implementer at which commit? Fix: say the messages name the implementer, and say land stays Git-only (consistent with the merge-may-refuse-what-land-accepts comment at 1112-1114) or test it.
