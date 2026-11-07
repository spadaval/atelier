---
schema: atelier.review
schema_version: 2
id: atelier-8ysu
title: 'LOW crates/atelier-cli/src/commands/merge.rs:341-343 merge.merged_tree passes when any shell.check node passed, and its reason then lists every merge_checks command as passed. With two shell checks where one fails, output reads ''FAIL shell.check'' alongside ''pass merge.merged_tree ... and passed: a; b'', which misstates the merged tree''s proof. Not a gate hole (the failing shell.check node still blocks). Fix: pass only when every shell.check node passed and at least one ran; keep the failing text naming the failed command.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: fe1ea4807f2c7d27d349a570d5d437928de37b09
created_at: 2026-10-17T06:35:36.455973429Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5)
    commit: 17718c33af9cc8b05ca8c453c09f30d66e7a6a02
    recorded_at: 2026-10-17T07:04:07.533077872Z
    resolution_commit: a3a10b23e016c844519dcf448264d93e4e30f9f9
    reason: merge.merged_tree passes only when at least one shell.check ran and every shell.check passed, and names only those; tested with one passing and one failing check.
    migrated_from: atelier-n1l7
---

LOW crates/atelier-cli/src/commands/merge.rs:341-343 merge.merged_tree passes when any shell.check node passed, and its reason then lists every merge_checks command as passed. With two shell checks where one fails, output reads 'FAIL shell.check' alongside 'pass merge.merged_tree ... and passed: a; b', which misstates the merged tree's proof. Not a gate hole (the failing shell.check node still blocks). Fix: pass only when every shell.check node passed and at least one ran; keep the failing text naming the failed command.
