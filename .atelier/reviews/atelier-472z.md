---
schema: atelier.review
schema_version: 2
id: atelier-472z
title: Low. crates/atelier-app/src/record_index.rs:360-363 still labels a blocker cycle 'relationships.blocks contains a cycle', but index.blocking now also holds blocked_by entries from relates, which is the default storage for new links. A user told to look in relationships.blocks will usually find nothing on the records that form the cycle. Fold into atelier-lehs (cycle errors list the cycle) or relabel as the blocker graph.
target: atelier-ihss
identity: Independent reviewer (Claude subagent)
commit: af8645378791a3ff66fe5d8ab2a5db7d9cd010bd
created_at: 2026-10-17T06:27:54.589034782Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5)
    commit: 92d8c5ebf2be304ee2f2faba58c35db9d2cc9465
    recorded_at: 2026-10-17T06:28:38.702955367Z
    follow_up: atelier-t18i
    reason: Error wording only; the cycle is still detected. Tracked in atelier-t18i.
    migrated_from: atelier-z6lh
---

Low. crates/atelier-app/src/record_index.rs:360-363 still labels a blocker cycle 'relationships.blocks contains a cycle', but index.blocking now also holds blocked_by entries from relates, which is the default storage for new links. A user told to look in relationships.blocks will usually find nothing on the records that form the cycle. Fold into atelier-lehs (cycle errors list the cycle) or relabel as the blocker graph.
