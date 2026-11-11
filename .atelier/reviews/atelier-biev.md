---
schema: atelier.review
schema_version: 2
id: atelier-biev
title: finding on atelier-wd1i @d41d3bcd
target: atelier-wd1i
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:52:33.150463391Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-wd1i.md:47-50 The plan runs the script through its final close-and-push before the independent real-checkout and conversion-CI checks, then orders rollback if any check fails. That is a reachable state F's planned rollback does not cover: its table reverts the conversion commit only, but close has since modified the new item file that commit introduced (next/crates/atelier-cli/src/plan.rs:503-508; psjw adds its commit). Reverting the conversion beneath the close can conflict and leaves the cutover recorded done before acceptance succeeded; none of F's six failure trials exercises this. Settle the finalization and late-failure policy in both plans: either perform the required acceptance before final close with an executable reviewed sequence, or journal/undo post-conversion commits in reverse order and prove rollback after close with both pushed and unpushed closing commits, including a failed conversion-CI result. Also make rollback success compare restored trees, not original commit IDs for repositories that require pushed reverts. The day must not require choosing how to undo a closed cutover.
