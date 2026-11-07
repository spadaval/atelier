---
schema: atelier.review
schema_version: 2
id: atelier-ig18
title: 'crates/atelier-core/src/lib.rs:364-382 (IssueSections::change_type): Background is the one optional section both body shapes now share, but change_type rebuilds the body via IssueBody::for_type(..., background: None), so ''atelier issue update <id> --issue-type mission'' (or mission -> task) silently deletes the Background section. Reproduced in a scratch repo: a task with Goal/Done when/Context/Background retyped to mission kept only Outcome and Success criteria; Background was gone from the file and from issue show, with no warning. That is data loss for exactly the shared research this change exists to hold. Fix: carry background across in change_type (extract it from either variant and set it on the new body), and extend the change_type unit test at crates/atelier-core/src/lib.rs:~663 to assert Background survives task -> mission -> task.'
target: atelier-by8u
identity: Independent reviewer (Claude subagent)
commit: 8d219563b31b3ca6885be30cdbc5c5b84ee204c4
created_at: 2026-10-16T00:53:58.934665673Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 3df28a5f40d2468ee052e4935daafaec8a0c0829
    recorded_at: 2026-10-16T00:55:47.557868234Z
    resolution_commit: 3df28a5f40d2468ee052e4935daafaec8a0c0829
    reason: Fixed in 3df28a5f40d2468ee052e4935daafaec8a0c0829
    migrated_from: atelier-3vfw
---

crates/atelier-core/src/lib.rs:364-382 (IssueSections::change_type): Background is the one optional section both body shapes now share, but change_type rebuilds the body via IssueBody::for_type(..., background: None), so 'atelier issue update <id> --issue-type mission' (or mission -> task) silently deletes the Background section. Reproduced in a scratch repo: a task with Goal/Done when/Context/Background retyped to mission kept only Outcome and Success criteria; Background was gone from the file and from issue show, with no warning. That is data loss for exactly the shared research this change exists to hold. Fix: carry background across in change_type (extract it from either variant and set it on the new body), and extend the change_type unit test at crates/atelier-core/src/lib.rs:~663 to assert Background survives task -> mission -> task.
