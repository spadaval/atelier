---
schema: atelier.review
schema_version: 1
id: atelier-rg6u
title: 'high: crates/atelier-cli/src/commands/session.rs:119-128 (push_checked_branch) pushes HEAD to whatever branch.<name>.merge names, by explicit refspec, without checking the destination is the current branch''s own name. A work branch created the common way, `git switch -c task/<id> origin/main`, tracks origin/main, so `atelier land --push` on it pushes unreviewed work straight to the remote base. Reproduced with a local bare remote: task/atelier-kmx5 tracking origin/main, one unreviewed code commit, land --push printed ''land: passed, pushed to origin/main'', exit 0, and the remote main now holds the unreviewed commit. A remote of ''.'' (local upstream) would likewise move a local branch such as main. The explicit refspec also bypasses push.default=simple, which refuses exactly this mismatch. Fix: refuse (e.g. land_push_upstream_mismatch) unless remote != ''.'' and the merge ref is refs/heads/<current branch>; add a test with an upstream of a different name.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 6b8b1ccb31042d1a3a7b167903f6a43b4a9acdf7
created_at: 2026-10-22T21:06:42.229194447Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/atelier-cli/src/commands/session.rs:119-128 (push_checked_branch) pushes HEAD to whatever branch.<name>.merge names, by explicit refspec, without checking the destination is the current branch's own name. A work branch created the common way, `git switch -c task/<id> origin/main`, tracks origin/main, so `atelier land --push` on it pushes unreviewed work straight to the remote base. Reproduced with a local bare remote: task/atelier-kmx5 tracking origin/main, one unreviewed code commit, land --push printed 'land: passed, pushed to origin/main', exit 0, and the remote main now holds the unreviewed commit. A remote of '.' (local upstream) would likewise move a local branch such as main. The explicit refspec also bypasses push.default=simple, which refuses exactly this mismatch. Fix: refuse (e.g. land_push_upstream_mismatch) unless remote != '.' and the merge ref is refs/heads/<current branch>; add a test with an upstream of a different name.
