---
schema: atelier.review
schema_version: 2
id: atelier-5rgt
title: 'low: crates/atelier-cli/src/commands/merge_sync.rs:380 The stale-approval line is ungrammatical in both branches: the format ''{} {} since {}'' with ''went stale at'' / ''was already stale at'' prints ''atelier-x went stale at since 59548c80'' (reproduced). This is the line the feature exists to print, and the test only checks the substring ''went stale at'', so it did not catch it. Fix: drop ''since'' (or ''at'') and assert the whole line in sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.538853388Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bb4bb0d18548d572de9ffbb93054e0a5bfcb63f5
    recorded_at: 2026-10-22T22:34:12.483885417Z
    resolution_commit: 12ffb6f0af9dc064a123b4c405102faa0c991255
    reason: Fixed in 12ffb6f0
    migrated_from: atelier-ex17
---

low: crates/atelier-cli/src/commands/merge_sync.rs:380 The stale-approval line is ungrammatical in both branches: the format '{} {} since {}' with 'went stale at' / 'was already stale at' prints 'atelier-x went stale at since 59548c80' (reproduced). This is the line the feature exists to print, and the test only checks the substring 'went stale at', so it did not catch it. Fix: drop 'since' (or 'at') and assert the whole line in sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution.
