---
schema: atelier.review
schema_version: 2
id: atelier-21rl
title: finding on atelier-8myy @00dc91fe
target: atelier-8myy
identity: gpt-6.1-sol
commit: 00dc91fefc0f7b7741630a4529891811155da5e0
created_at: 2026-11-11T02:11:10.240688629Z
event:
  event: finding
  recommendation: defer
---

R6 low: next/crates/atelier-cli/src/plan.rs:344 The exclusion warning misses an item whose mission membership exists only in the index. Reproduced in a scratch repository: commit mission demo-m1ss and draft demo-ab12 with mission: null; change the item to mission: demo-m1ss and git add it; restore its working file to mission: null without resetting the index; review demo-m1ss exits 0 with no exclusion line although git diff --cached shows the staged assignment. changed_since now finds the staged path, but in_plan checks membership only in the working file and reviewed commit, so filters it out. The committed brief and recorded commit remain correct; only the warning is incomplete. Include staged membership when classifying changed item directories and cover this index-only assignment. DEFER under the review procedure rule that a new finding on the fix of a low-severity finding is deferred unless trivial or a regression introduced by that fix: this extends R4 (atelier-o94f), is not a regression from adding cached paths, and snapshot safety holds. No fix_now recommendation.
