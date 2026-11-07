---
schema: atelier.review
schema_version: 2
id: atelier-of4r
title: 'crates/atelier-cli/src/commands/session.rs:317 diffs each unpushed base commit only against its first parent (commit^). A merge commit on the base made by an ordinary ''git pull'' (pull.rebase=false) after someone else pushed reviewed code therefore shows all of upstream''s code paths as changed and land refuses with land_base_unreviewed. Reproduced: bare upstream, clone B pushes a code commit, clone A commits a record-only ''file issue'' on main then ''git pull --no-edit'' -> ''Error: land_base_unreviewed: commit e5089404 on main changes app without recorded review''. land runs on every Stop, so this blocks stopping in an ordinary state, and the advice (move the commit onto a work branch) is wrong for a merge. Fix: for a merge commit, count only paths that differ from every parent (git diff-tree -c/--cc --name-only, or intersect changed_paths against each parent), so only content the merge itself introduces is checked; add a test with a pull-merge on the base.'
target: atelier-3mjv
identity: Independent reviewer (Claude subagent)
commit: f9b76ba6c8397e90f916a6aff44f8f4915d2a032
created_at: 2026-10-15T23:18:20.944971762Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: e37f556bdb59ce8ac006a48fdfcd436dd236c081
    recorded_at: 2026-10-15T23:26:24.792980970Z
    resolution_commit: e37f556bdb59ce8ac006a48fdfcd436dd236c081
    reason: Fixed in e37f556bdb59ce8ac006a48fdfcd436dd236c081 with tests
    migrated_from: atelier-0h21
---

crates/atelier-cli/src/commands/session.rs:317 diffs each unpushed base commit only against its first parent (commit^). A merge commit on the base made by an ordinary 'git pull' (pull.rebase=false) after someone else pushed reviewed code therefore shows all of upstream's code paths as changed and land refuses with land_base_unreviewed. Reproduced: bare upstream, clone B pushes a code commit, clone A commits a record-only 'file issue' on main then 'git pull --no-edit' -> 'Error: land_base_unreviewed: commit e5089404 on main changes app without recorded review'. land runs on every Stop, so this blocks stopping in an ordinary state, and the advice (move the commit onto a work branch) is wrong for a merge. Fix: for a merge commit, count only paths that differ from every parent (git diff-tree -c/--cc --name-only, or intersect changed_paths against each parent), so only content the merge itself introduces is checked; add a test with a pull-merge on the base.
