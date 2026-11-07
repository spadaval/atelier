---
schema: atelier.review
schema_version: 2
id: atelier-l82f
title: finding on atelier-l7nz @e6210fb6
target: atelier-l7nz
identity: Independent reviewer (Claude Sonnet subagent)
commit: e6210fb6abda8f05565720c55d5ce1125e2f69c2
created_at: 2026-10-23T16:20:23.504381129Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 71471565ea562d20206bb9753f7982aa674f3e19
    recorded_at: 2026-10-23T16:21:00.301299059Z
    follow_up: atelier-4urj
    reason: No divergence found and a test pins agreement; unifying changes gate code, which this increment must not.
    migrated_from: atelier-vpxg
---

low: crates/atelier-app/src/evidence_freshness.rs: item_proof is a third copy of the pass/fail supersession rule beside evaluate_issue_gate and newest_failure; the gates do not call it (only the CLI does), contrary to the review brief's 'now backs the gates' yes/no check'. I found no divergence across pass-then-code, pass-then-fail, fail-then-pass, dirty tree, attach-reused and manual receipts, but agreement rests on a parity test rather than shared code; list also ignores a gate's kind filter. Also 'evidence show' judges only the first issue target, so an attach-reused receipt's standing on its other items is not shown. Consider having evaluate_issue_gate/newest_failure consume item_proof later.
