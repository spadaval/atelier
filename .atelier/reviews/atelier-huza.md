---
schema: atelier.review
schema_version: 1
id: atelier-huza
title: 'crates/atelier-cli/src/commands/session.rs:425: the note ends ''a clean merge keeps an approval'', which is false. approval_is_current walks edges from the approval pin to HEAD, and the merge commit''s edge is taken from the branch-side parent, so its paths are the base''s changes. allowed_edge rejects any non-record path. Reproduced with the built binary: terminal owner approved, base gains one code commit (''other''), land passes with the note, then ''git merge main'' (clean), and land/merge fail with review.current ''no current independent approval''. An agent that follows the note expects to keep its approval and is surprised. Drop the clause or say fresh review is needed (agree with atelier-j48s), and add the merge-then-land step to the test.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.055807048Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/session.rs:425: the note ends 'a clean merge keeps an approval', which is false. approval_is_current walks edges from the approval pin to HEAD, and the merge commit's edge is taken from the branch-side parent, so its paths are the base's changes. allowed_edge rejects any non-record path. Reproduced with the built binary: terminal owner approved, base gains one code commit ('other'), land passes with the note, then 'git merge main' (clean), and land/merge fail with review.current 'no current independent approval'. An agent that follows the note expects to keep its approval and is surprised. Drop the clause or say fresh review is needed (agree with atelier-j48s), and add the merge-then-land step to the test.
