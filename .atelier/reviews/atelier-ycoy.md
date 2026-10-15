---
schema: atelier.review
schema_version: 1
id: atelier-ycoy
title: 'crates/atelier-cli/src/commands/git.rs:112 (unmerged_done_label), used by session.rs:80 and status.rs:64: for the current checkout the status comes from the live working-tree record but the merged test uses HEAD. When the done claim is uncommitted and HEAD is already in the base (fresh work branch with no commits yet, or right after start), prime and status print ''[done, merged into <base>]'' though the base has no done claim at all. Reproduced: scratch repo with base trunk, atelier start, close without committing -> ''Owner: atelier-vk77 [done, merged into trunk]'' with '' M .atelier/issues/atelier-vk77.md'' dirty. Suggested fix: only report ''merged'' when HEAD is an ancestor of the base AND the owner record is not dirty (or check that the base tip''s copy of the record has a done-category status); otherwise report ''not merged'' (or ''uncommitted''). Add a session.rs test for the uncommitted case.'
target: atelier-9k7o
identity: Independent reviewer (Claude subagent)
commit: 841273234e11924575876c5ae52c5356d0dc27ed
created_at: 2026-10-15T23:10:24.131215776Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/git.rs:112 (unmerged_done_label), used by session.rs:80 and status.rs:64: for the current checkout the status comes from the live working-tree record but the merged test uses HEAD. When the done claim is uncommitted and HEAD is already in the base (fresh work branch with no commits yet, or right after start), prime and status print '[done, merged into <base>]' though the base has no done claim at all. Reproduced: scratch repo with base trunk, atelier start, close without committing -> 'Owner: atelier-vk77 [done, merged into trunk]' with ' M .atelier/issues/atelier-vk77.md' dirty. Suggested fix: only report 'merged' when HEAD is an ancestor of the base AND the owner record is not dirty (or check that the base tip's copy of the record has a done-category status); otherwise report 'not merged' (or 'uncommitted'). Add a session.rs test for the uncommitted case.
