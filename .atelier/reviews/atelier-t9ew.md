---
schema: atelier.review
schema_version: 2
id: atelier-t9ew
title: 'low: crates/atelier-cli/src/commands/merge.rs:530-541,586-589 The R1 fix checks for a records-only base move just before printing the gate report, but the authoritative base check runs again after the canonical mutation locks are taken (line 588). If the base moves by a records-only commit in that window (after the pre-print resolve, before the post-lock resolve, which includes any wait for the mutation locks), merge_once has already printed and still returns RetryOnMovedBase, so --json stdout again holds two arrays and human output repeats the gate lines. The window is now microseconds plus lock wait instead of the whole check run, so this is far narrower than atelier-grmb. Possible fix if it matters: take the locks before the pre-print check, or buffer the report and print it only from the attempt that returns.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: 4899924eee8c4553837f8511b68a10a557fec750
created_at: 2026-10-22T20:59:02.033059588Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 8bb5f9c60d21e237cf04544344c6976efeb8dad2
    recorded_at: 2026-10-22T20:59:30.826955977Z
    follow_up: atelier-1hjj
    reason: Microsecond window after the R1 fix; tracked in atelier-1hjj.
    migrated_from: atelier-r9pi
---

low: crates/atelier-cli/src/commands/merge.rs:530-541,586-589 The R1 fix checks for a records-only base move just before printing the gate report, but the authoritative base check runs again after the canonical mutation locks are taken (line 588). If the base moves by a records-only commit in that window (after the pre-print resolve, before the post-lock resolve, which includes any wait for the mutation locks), merge_once has already printed and still returns RetryOnMovedBase, so --json stdout again holds two arrays and human output repeats the gate lines. The window is now microseconds plus lock wait instead of the whole check run, so this is far narrower than atelier-grmb. Possible fix if it matters: take the locks before the pre-print check, or buffer the report and print it only from the attempt that returns.
