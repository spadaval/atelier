---
schema: atelier.review
schema_version: 2
id: atelier-5x68
title: 'low: crates/atelier-workflow/src/lib.rs:621 Before this change an omitted merge_strategy parsed as squash (master: value.unwrap_or("squash")), so prune.rs:490 accepted squash-equivalent branches as integrated for such repositories. Now omission yields fast_forward_only or merge_commit, so a repository that relied on the omitted value for prune''s squash detection stops pruning squash-merged branches. The documented default was already fast_forward_only, merge refused squash, and this repository sets merge_strategy explicitly, so the impact is limited to undocumented behaviour; noting it so the 2yl7 compatibility note (which mentions only merge) is complete.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:28.049026744Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: 748d25423e2759ed1ed5ae8f9fc5244ae1cade4c
    recorded_at: 2026-10-22T20:53:03.449479750Z
    reason: The documented default was already fast_forward_only and this repository sets merge_strategy explicitly; prune's squash detection follows the declared strategy.
    migrated_from: atelier-9o6e
---

low: crates/atelier-workflow/src/lib.rs:621 Before this change an omitted merge_strategy parsed as squash (master: value.unwrap_or("squash")), so prune.rs:490 accepted squash-equivalent branches as integrated for such repositories. Now omission yields fast_forward_only or merge_commit, so a repository that relied on the omitted value for prune's squash detection stops pruning squash-merged branches. The documented default was already fast_forward_only, merge refused squash, and this repository sets merge_strategy explicitly, so the impact is limited to undocumented behaviour; noting it so the 2yl7 compatibility note (which mentions only merge) is complete.
