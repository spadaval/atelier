---
schema: atelier.review
schema_version: 1
id: atelier-8ysu
title: 'LOW crates/atelier-cli/src/commands/merge.rs:341-343 merge.merged_tree passes when any shell.check node passed, and its reason then lists every merge_checks command as passed. With two shell checks where one fails, output reads ''FAIL shell.check'' alongside ''pass merge.merged_tree ... and passed: a; b'', which misstates the merged tree''s proof. Not a gate hole (the failing shell.check node still blocks). Fix: pass only when every shell.check node passed and at least one ran; keep the failing text naming the failed command.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: fe1ea4807f2c7d27d349a570d5d437928de37b09
created_at: 2026-10-17T06:35:36.455973429Z
event:
  event: finding
  recommendation: fix_now
---

LOW crates/atelier-cli/src/commands/merge.rs:341-343 merge.merged_tree passes when any shell.check node passed, and its reason then lists every merge_checks command as passed. With two shell checks where one fails, output reads 'FAIL shell.check' alongside 'pass merge.merged_tree ... and passed: a; b', which misstates the merged tree's proof. Not a gate hole (the failing shell.check node still blocks). Fix: pass only when every shell.check node passed and at least one ran; keep the failing text naming the failed command.
