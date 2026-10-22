---
schema: atelier.review
schema_version: 1
id: atelier-t9ew
title: 'low: crates/atelier-cli/src/commands/merge.rs:530-541,586-589 The R1 fix checks for a records-only base move just before printing the gate report, but the authoritative base check runs again after the canonical mutation locks are taken (line 588). If the base moves by a records-only commit in that window (after the pre-print resolve, before the post-lock resolve, which includes any wait for the mutation locks), merge_once has already printed and still returns RetryOnMovedBase, so --json stdout again holds two arrays and human output repeats the gate lines. The window is now microseconds plus lock wait instead of the whole check run, so this is far narrower than atelier-grmb. Possible fix if it matters: take the locks before the pre-print check, or buffer the report and print it only from the attempt that returns.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: 4899924eee8c4553837f8511b68a10a557fec750
created_at: 2026-10-22T20:59:02.033059588Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/merge.rs:530-541,586-589 The R1 fix checks for a records-only base move just before printing the gate report, but the authoritative base check runs again after the canonical mutation locks are taken (line 588). If the base moves by a records-only commit in that window (after the pre-print resolve, before the post-lock resolve, which includes any wait for the mutation locks), merge_once has already printed and still returns RetryOnMovedBase, so --json stdout again holds two arrays and human output repeats the gate lines. The window is now microseconds plus lock wait instead of the whole check run, so this is far narrower than atelier-grmb. Possible fix if it matters: take the locks before the pre-print check, or buffer the report and print it only from the attempt that returns.
