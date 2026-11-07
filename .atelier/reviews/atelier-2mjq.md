---
schema: atelier.review
schema_version: 2
id: atelier-2mjq
title: 'low: crates/atelier-app/src/review_gate.rs:199 approval_stale_edge''s doc (and docs/product/reviews-and-merge.md ''the first commit that made it stale'') says it returns the first commit after the pin that makes the approval stale, but read.edges is rev-list order, newest first, so it returns the newest failing edge and only that edge''s paths. Repro: approve, commit unreviewed b.rs, sync with a conflict on src.rs, resolve, sync again: the output says ''was already stale at since <merge>'' and ''Review 1 path: src.rs''. b.rs, the commit that actually made it stale, is not named. Whenever the approval was current before the sync the only failing edge is the sync merge, so the common path is right. The already-stale path misnames the commit and understates what to review. Fix: walk the edges oldest first (or collect the paths of every failing edge), keeping approval_is_current_to''s bool unchanged, and add a test with a pre-sync unreviewed commit.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.612155003Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bb4bb0d18548d572de9ffbb93054e0a5bfcb63f5
    recorded_at: 2026-10-22T22:34:12.663150190Z
    resolution_commit: 12ffb6f0af9dc064a123b4c405102faa0c991255
    reason: Fixed in 12ffb6f0
    migrated_from: atelier-6vl6
---

low: crates/atelier-app/src/review_gate.rs:199 approval_stale_edge's doc (and docs/product/reviews-and-merge.md 'the first commit that made it stale') says it returns the first commit after the pin that makes the approval stale, but read.edges is rev-list order, newest first, so it returns the newest failing edge and only that edge's paths. Repro: approve, commit unreviewed b.rs, sync with a conflict on src.rs, resolve, sync again: the output says 'was already stale at since <merge>' and 'Review 1 path: src.rs'. b.rs, the commit that actually made it stale, is not named. Whenever the approval was current before the sync the only failing edge is the sync merge, so the common path is right. The already-stale path misnames the commit and understates what to review. Fix: walk the edges oldest first (or collect the paths of every failing edge), keeping approval_is_current_to's bool unchanged, and add a test with a pre-sync unreviewed commit.
