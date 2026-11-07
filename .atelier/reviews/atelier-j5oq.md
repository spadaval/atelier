---
schema: atelier.review
schema_version: 2
id: atelier-j5oq
title: finding on atelier-hwxm @43db25b6
target: atelier-hwxm
identity: lighter-workflow reviewer for atelier-hwxm (claude-opus-5-5)
commit: 43db25b6d87c91db4757480ab940b3b82abce0e7
created_at: 2026-11-07T01:29:37.188339722Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: a60de2c8a4b85ef5a4d7533795ecd1daf721a47e
    recorded_at: 2026-11-07T01:33:01.502464101Z
    resolution_commit: a60de2c8a4b85ef5a4d7533795ecd1daf721a47e
    reason: The review_append_only refusal for a merge that removed a migrated base's event while the branch holds a later one now names the recovery (leave the merge out, git merge --no-commit <base>, atelier migrate dispositions --apply, commit), and ADR 0025 and reviews-and-merge.md give that sequence; no acceptance rule changed. a_migrated_base_merges_into_a_branch_with_events_by_migrating_the_merge checks that the refusal names it, that its printed fix runs, and that the sequence leaves a branch land and check accept.
---

low: crates/atelier-app/src/review_gate.rs:371 (folded_by) The path ADR 0025 decision 6 and docs/product/reviews-and-merge.md:95 document, 'events that continue a folded disposition, as a branch recorded with an older Atelier brings in, replace it', is now refused when it happens by merge. Reproduced (fixture fx4): main has finding F with old event E1; branch side records E2 superseding E1 with the old binary; main runs migrate --apply and commits; side runs a plain git merge main. The merge's first-parent edge deletes E1 while E2 remains, so folded_by is false and doctor refuses review_append_only for E1, saying a later commit cannot undo it (the branch must be reset). The read result itself is right (F shows E2, migrated_from E2). git merge --no-commit main, then migrate dispositions --apply, then commit, passes. Fix, either: accept the deletion when the finding at the commit carries a field and the finding's remaining events continue from it (the fold at that commit succeeds), or document that such a merge must run migrate before committing (and add a merge case to session.rs). The plain addition of a successor event that the existing test covers still passes.
