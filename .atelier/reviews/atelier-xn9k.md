---
schema: atelier.review
schema_version: 2
id: atelier-xn9k
title: finding on atelier-8myy @5f7737c1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 5f7737c1d5c68bc04379a944f589437a75e86734
created_at: 2026-11-11T02:00:34.895177897Z
event:
  event: finding
  recommendation: fix_now
---

R2 medium: next/crates/atelier-cli/src/views.rs:51 A done base record can still be hidden by a broken record on its already-merged branch. The checkout re-read is suppressed only when !branch.merged. Reproduced: commit status: broken on demo-ab12; on main merge that branch with --no-commit --no-ff, write the item as done and commit; in the still-checked-out demo-ab12 worktree, status succeeds as done but show demo-ab12 exits 2 parsing the branch's broken item.md. The second Done when line says the base's terminal status wins whatever its branch holds. Make settlement overlays unable to turn the base's valid terminal record into a parse failure, including merged own branches; retain healthy settlement visibility and cover this case.
