---
schema: atelier.review
schema_version: 1
id: atelier-eqrz
title: 'medium: crates/atelier-cli/src/commands/start.rs:36-40: with --base, the issue id is resolved and the owner branch derived from the current checkout''s records before the base snapshot is read. From a branch or detached HEAD that lacks the issue''s record (e.g. another owner''s branch forked before the item was planned on master, or a detached older commit), `atelier start <id> --base main` fails with ''issue <id> not found'', although the docs (work-model.md, decision note) say the records are read from the base. Reproduced: on branch ''old'' (before the plan commit) and on detached main~1, start atelier-gz3q --base main exits 1 with ''issue atelier-gz3q not found''. This misses yc4d''s Done when (''works from a detached HEAD or any branch''). Fix: when --base is given, resolve the id and derive the branch from the base snapshot; add a test from a checkout without the record.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 6b8b1ccb31042d1a3a7b167903f6a43b4a9acdf7
created_at: 2026-10-22T21:06:42.372048519Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/start.rs:36-40: with --base, the issue id is resolved and the owner branch derived from the current checkout's records before the base snapshot is read. From a branch or detached HEAD that lacks the issue's record (e.g. another owner's branch forked before the item was planned on master, or a detached older commit), `atelier start <id> --base main` fails with 'issue <id> not found', although the docs (work-model.md, decision note) say the records are read from the base. Reproduced: on branch 'old' (before the plan commit) and on detached main~1, start atelier-gz3q --base main exits 1 with 'issue atelier-gz3q not found'. This misses yc4d's Done when ('works from a detached HEAD or any branch'). Fix: when --base is given, resolve the id and derive the branch from the base snapshot; add a test from a checkout without the record.
