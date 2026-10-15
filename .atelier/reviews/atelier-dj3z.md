---
schema: atelier.review
schema_version: 1
id: atelier-dj3z
title: 'crates/atelier-app/src/init.rs:130-136: a linked worktree of a repo that uses a separate git dir, with no origin, still gets its own directory name. Reproduced: sepwork created with --separate-git-dir sep.gitdir, then a worktree sep-feat; init in sep-feat gives ''sep-feat''. The common dir is not named .git and is not bare, so neither the main-worktree source nor the bare source applies. Fixing it would mean reading the main worktree from ''git worktree list --porcelain''. This needs a separate git dir, a linked worktree and no origin all at once, and the printed slug plus --slug cover it, so no action.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 56a5abb90e09f8c9710ab502b4ed448e69faacdc
created_at: 2026-10-15T23:25:03.635829589Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/init.rs:130-136: a linked worktree of a repo that uses a separate git dir, with no origin, still gets its own directory name. Reproduced: sepwork created with --separate-git-dir sep.gitdir, then a worktree sep-feat; init in sep-feat gives 'sep-feat'. The common dir is not named .git and is not bare, so neither the main-worktree source nor the bare source applies. Fixing it would mean reading the main worktree from 'git worktree list --porcelain'. This needs a separate git dir, a linked worktree and no origin all at once, and the printed slug plus --slug cover it, so no action.
