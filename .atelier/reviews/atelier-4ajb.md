---
schema: atelier.review
schema_version: 2
id: atelier-4ajb
title: finding on atelier-syyj @24388025
target: atelier-syyj
identity: gpt-6.1-sol
commit: 2438802514475400e885b0ba2335b18e56fabd5a
created_at: 2026-11-10T18:25:20.724014216Z
event:
  event: finding
  recommendation: fix_now
---

high: next/crates/atelier-cli/src/merge.rs:185 Protect ignored local files before fast-forwarding the base checkout. The cleanliness checks at lines 99 and 259 inspect only tracked files, and git merge --ff-only defaults to overwriting ignored files. Reproduced offline at the pinned head: approve a branch that adds local.txt, ignore that path through the shared .git/info/exclude, and put different valuable contents at local.txt in a clean main worktree. atelier merge exits 0, marks the item done, and silently replaces the base checkout file with the branch contents. The old merge had an untracked-overwrite guard; omitting it loses local data. Pass --no-overwrite-ignore to the fast-forward merge or otherwise refuse any overwrite of untracked or ignored paths, and add a scenario asserting the base ref and file contents remain unchanged.
