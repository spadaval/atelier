---
schema: atelier.review
schema_version: 1
id: atelier-86y8
title: 'crates/atelier-app/src/init.rs:124-127: a linked worktree of a bare repository (layout proj.git + proj-feature) with no origin has a common dir that is not named ''.git'', so main_worktree is None and the slug falls back to ''proj-feature'', which is the bug this record describes, in the bare-repo worktree layout. Reproduced in /tmp. Suggested fix: when the common dir is bare (git rev-parse --is-bare-repository run against it, or name not ''.git''), use its directory name with a trailing ''.git'' removed. The done-when (origin, then main worktree) is met, and bare layouts usually have an origin, so this can be deferred.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 1abac3dcb9b83de0291f40d840a970264b557a31
created_at: 2026-10-15T23:17:24.139779376Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/init.rs:124-127: a linked worktree of a bare repository (layout proj.git + proj-feature) with no origin has a common dir that is not named '.git', so main_worktree is None and the slug falls back to 'proj-feature', which is the bug this record describes, in the bare-repo worktree layout. Reproduced in /tmp. Suggested fix: when the common dir is bare (git rev-parse --is-bare-repository run against it, or name not '.git'), use its directory name with a trailing '.git' removed. The done-when (origin, then main worktree) is met, and bare layouts usually have an origin, so this can be deferred.
