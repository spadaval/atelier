---
schema: atelier.review
schema_version: 2
id: atelier-huza
title: 'crates/atelier-cli/src/commands/session.rs:425: the note ends ''a clean merge keeps an approval'', which is false. approval_is_current walks edges from the approval pin to HEAD, and the merge commit''s edge is taken from the branch-side parent, so its paths are the base''s changes. allowed_edge rejects any non-record path. Reproduced with the built binary: terminal owner approved, base gains one code commit (''other''), land passes with the note, then ''git merge main'' (clean), and land/merge fail with review.current ''no current independent approval''. An agent that follows the note expects to keep its approval and is surprised. Drop the clause or say fresh review is needed (agree with atelier-j48s), and add the merge-then-land step to the test.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.055807048Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 5c2e3b0e7fec9f639eb9cfc08cb680b54925c115
    recorded_at: 2026-10-16T18:41:55.026353352Z
    resolution_commit: 25a5eb219903546052b8a4425bc0f4edcc87bd11
    reason: 'Resolved: with atelier-n7g1 merged, a clean merge of the base keeps an approval current, so the note''s clause is true. land_on_a_branch_behind_its_base_hands_off_and_says_to_merge_the_base now approves while behind, sees merge_diverged, runs git merge main, and then land (note gone) and atelier merge both succeed.'
    migrated_from: atelier-je1m
---

crates/atelier-cli/src/commands/session.rs:425: the note ends 'a clean merge keeps an approval', which is false. approval_is_current walks edges from the approval pin to HEAD, and the merge commit's edge is taken from the branch-side parent, so its paths are the base's changes. allowed_edge rejects any non-record path. Reproduced with the built binary: terminal owner approved, base gains one code commit ('other'), land passes with the note, then 'git merge main' (clean), and land/merge fail with review.current 'no current independent approval'. An agent that follows the note expects to keep its approval and is surprised. Drop the clause or say fresh review is needed (agree with atelier-j48s), and add the merge-then-land step to the test.
