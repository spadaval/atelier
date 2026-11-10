---
schema: atelier.review
schema_version: 2
id: atelier-nioh
title: finding on atelier-syyj @2d35d0a5
target: atelier-syyj
identity: gpt-6.1-sol
commit: 2d35d0a574fe64d9c96455410160e593e9fe16c2
created_at: 2026-11-10T17:50:55.818904522Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/commands.rs:24 Show configured merge checks and the CI requirement in status. The Goal requires three checks an agent could already see in status, but status never reads the base config and exposes only unmet act conditions. Reproduced with an approved item and a configured test -f hello.txt merge check: status ends with Status: approved and Then: atelier merge, with no mention of that command or a remaining merge-check gate. The new architecture text at docs/strategies/redesign-architecture.md:361 explicitly moves the checks and CI out of status; matching the implementation by narrowing the documentation does not meet the Goal. List the commands from the base config and any configured CI requirement as checks merge still needs to establish, without running shell commands or looking up CI from status, and add a scenario test. Verdict: merge after both fix-now findings are fixed and independently re-reviewed; no approval of this head. Sizes: complexity high (Git snapshots, temporary worktrees, serialisation and forge integration), value high (closes the approved-to-done loop), risk high (the separate snapshot finding silently merges unreviewed source). Contract and Fitness fail at this head. Added moving parts otherwise trace to the requested merge, safety, forge and retry behaviours. No open product question; residual risk is that real GitHub and network behaviour were not exercised, as required by this offline review.
