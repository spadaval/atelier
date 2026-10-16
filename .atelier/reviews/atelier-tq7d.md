---
schema: atelier.review
schema_version: 1
id: atelier-tq7d
title: 'Coverage and wording gaps. (1) crates/atelier-cli/tests/session.rs:2226 tests only passing cases while behind. Nothing shows that the fork-point base still fails a terminal owner with no approval, or that a done claim elsewhere is judged against the fork. The comment ''Completed work is still checked'' is not asserted. (2) session.rs:337 land_base_unrelated is untested, prints ''refs/heads/main'' rather than the branch name, and gives no recovery step. That step was the original complaint in this issue. Reproduced with an orphan owner branch: ''land_base_unrelated: refs/heads/main shares no history with HEAD / Caused by: git merge-base <a> <b> failed:''.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.099414671Z
event:
  event: finding
  recommendation: defer
---

Coverage and wording gaps. (1) crates/atelier-cli/tests/session.rs:2226 tests only passing cases while behind. Nothing shows that the fork-point base still fails a terminal owner with no approval, or that a done claim elsewhere is judged against the fork. The comment 'Completed work is still checked' is not asserted. (2) session.rs:337 land_base_unrelated is untested, prints 'refs/heads/main' rather than the branch name, and gives no recovery step. That step was the original complaint in this issue. Reproduced with an orphan owner branch: 'land_base_unrelated: refs/heads/main shares no history with HEAD / Caused by: git merge-base <a> <b> failed:'.
