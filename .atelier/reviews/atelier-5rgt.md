---
schema: atelier.review
schema_version: 1
id: atelier-5rgt
title: 'low: crates/atelier-cli/src/commands/merge_sync.rs:380 The stale-approval line is ungrammatical in both branches: the format ''{} {} since {}'' with ''went stale at'' / ''was already stale at'' prints ''atelier-x went stale at since 59548c80'' (reproduced). This is the line the feature exists to print, and the test only checks the substring ''went stale at'', so it did not catch it. Fix: drop ''since'' (or ''at'') and assert the whole line in sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.538853388Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/commands/merge_sync.rs:380 The stale-approval line is ungrammatical in both branches: the format '{} {} since {}' with 'went stale at' / 'was already stale at' prints 'atelier-x went stale at since 59548c80' (reproduced). This is the line the feature exists to print, and the test only checks the substring 'went stale at', so it did not catch it. Fix: drop 'since' (or 'at') and assert the whole line in sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution.
