---
schema: atelier.item
schema_version: 3
id: atelier-lkf5
title: Warn about an item whose mission assignment is only staged when a plan review opens
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T02:11:58.221236410Z
---

## Goal

Deferred from finding atelier-21rl on atelier-8myy @00dc91fe, by `atelier review defer`.

R6 low: crates/atelier-cli/src/plan.rs:344 The exclusion warning misses an item whose mission membership exists only in the index. Reproduced in a scratch repository: commit mission demo-m1ss and draft demo-ab12 with mission: null; change the item to mission: demo-m1ss and git add it; restore its working file to mission: null without resetting the index; review demo-m1ss exits 0 with no exclusion line although git diff --cached shows the staged assignment. changed_since now finds the staged path, but in_plan checks membership only in the working file and reviewed commit, so filters it out. The committed brief and recorded commit remain correct; only the warning is incomplete. Include staged membership when classifying changed item directories and cover this index-only assignment. DEFER under the review procedure rule that a new finding on the fix of a low-severity finding is deferred unless trivial or a regression introduced by that fix: this extends R4 (atelier-o94f), is not a regression from adding cached paths, and snapshot safety holds. No fix_now recommendation.

## Done when

- [ ] `atelier review <mission>` prints the exclusion line for an item whose mission assignment is staged but not in its working file, and a test covers that case.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-dd3a (Deferred review findings on atelier-8myy)
- discovered_from review: atelier-21rl
