---
schema: atelier.review
schema_version: 1
id: atelier-qp34
title: 'Commit 76d7af94 removes the false premise that isolation would drop the eol and text behaviour ordinary merges rely on, from both Design item 1 and the Close Reason. The new reason holds up. A configured driver is the repository''s chosen merge. Isolating fails closed: on git 2.53, with --attr-source set to the empty tree, the merge=ours and merge=union files conflict, and dropping the driver config gives a different text merge, so a driver-resolved base merge would no longer reproduce and would need renewed review. Keeping the documented caveat for that reason is a decision 09wh allowed.'
target: atelier-7hh4
identity: Independent reviewer (Claude subagent)
commit: 76d7af940461b7572283b461b9ec56f28cafd73a
created_at: 2026-10-16T20:47:24.446655765Z
event:
  event: disposition
  finding: atelier-09wh
  decision: fix_now
  supersedes: null
  resolution_commit: 76d7af940461b7572283b461b9ec56f28cafd73a
  follow_up: null
---

Commit 76d7af94 removes the false premise that isolation would drop the eol and text behaviour ordinary merges rely on, from both Design item 1 and the Close Reason. The new reason holds up. A configured driver is the repository's chosen merge. Isolating fails closed: on git 2.53, with --attr-source set to the empty tree, the merge=ours and merge=union files conflict, and dropping the driver config gives a different text merge, so a driver-resolved base merge would no longer reproduce and would need renewed review. Keeping the documented caveat for that reason is a decision 09wh allowed.
