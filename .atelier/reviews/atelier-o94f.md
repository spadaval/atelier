---
schema: atelier.review
schema_version: 2
id: atelier-o94f
title: finding on atelier-8myy @5f7737c1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 5f7737c1d5c68bc04379a944f589437a75e86734
created_at: 2026-11-11T02:00:35.719099231Z
event:
  event: finding
  recommendation: fix_now
---

R4 low: next/crates/atelier-cli/src/review.rs:186 The new exclusion line misses staged-only plan changes. plan_changed() uses git::changed_since(), which diffs the commit against the working files and adds untracked paths, but never checks the index. Reproduced: commit a mission, edit its Outcome, git add its mission.md, restore the working file to HEAD while retaining the staged edit, then review the mission. The command succeeds and prints the committed Outcome but no 'this review does not cover those changes' line, although git diff --cached shows the plan edit. Include staged differences as well as working-tree/untracked differences and deduplicate paths; add this index/worktree-divergence case to the first Done when test.
