---
schema: atelier.review
schema_version: 2
id: atelier-ouv8
title: 'crates/atelier-app/src/lint.rs:204 (two missions on one strategy): if missions A and B both link docs/strategies/x.md and x.md says `mission: A`, B gets a finding whose suggested fix is ''set `mission: B`''. Following that advice moves the finding to A. Scenario: a done or superseded mission and its successor share a strategy. Only ''remove the key'' resolves it. Suggested follow-up: when several missions link one strategy, name them all in the finding and recommend removing the key; or skip missions whose status is done.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.831289832Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    recorded_at: 2026-10-15T23:59:22.293496089Z
    resolution_commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    reason: Fixed in 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    migrated_from: atelier-ln67
---

crates/atelier-app/src/lint.rs:204 (two missions on one strategy): if missions A and B both link docs/strategies/x.md and x.md says `mission: A`, B gets a finding whose suggested fix is 'set `mission: B`'. Following that advice moves the finding to A. Scenario: a done or superseded mission and its successor share a strategy. Only 'remove the key' resolves it. Suggested follow-up: when several missions link one strategy, name them all in the finding and recommend removing the key; or skip missions whose status is done.
