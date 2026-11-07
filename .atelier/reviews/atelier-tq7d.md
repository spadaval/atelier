---
schema: atelier.review
schema_version: 2
id: atelier-tq7d
title: 'Coverage and wording gaps. (1) crates/atelier-cli/tests/session.rs:2226 tests only passing cases while behind. Nothing shows that the fork-point base still fails a terminal owner with no approval, or that a done claim elsewhere is judged against the fork. The comment ''Completed work is still checked'' is not asserted. (2) session.rs:337 land_base_unrelated is untested, prints ''refs/heads/main'' rather than the branch name, and gives no recovery step. That step was the original complaint in this issue. Reproduced with an orphan owner branch: ''land_base_unrelated: refs/heads/main shares no history with HEAD / Caused by: git merge-base <a> <b> failed:''.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.099414671Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 5c2e3b0e7fec9f639eb9cfc08cb680b54925c115
    recorded_at: 2026-10-16T18:41:55.115824374Z
    resolution_commit: 25a5eb219903546052b8a4425bc0f4edcc87bd11
    reason: 'Resolved: the behind test now covers a gate failing while behind. land_base_unrelated strips refs/heads/, names main, and gives a recovery step (switch to the owner''s work branch, or atelier start <id> from the base). land_names_a_base_with_no_shared_history covers it with an orphan branch.'
    migrated_from: atelier-1e0g
---

Coverage and wording gaps. (1) crates/atelier-cli/tests/session.rs:2226 tests only passing cases while behind. Nothing shows that the fork-point base still fails a terminal owner with no approval, or that a done claim elsewhere is judged against the fork. The comment 'Completed work is still checked' is not asserted. (2) session.rs:337 land_base_unrelated is untested, prints 'refs/heads/main' rather than the branch name, and gives no recovery step. That step was the original complaint in this issue. Reproduced with an orphan owner branch: 'land_base_unrelated: refs/heads/main shares no history with HEAD / Caused by: git merge-base <a> <b> failed:'.
