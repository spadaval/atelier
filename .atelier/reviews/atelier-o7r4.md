---
schema: atelier.review
schema_version: 2
id: atelier-o7r4
title: 'low: crates/atelier-app/src/review_gate.rs:206 approval_is_current_to now runs approval_stale_edge, which on every stale merge edge also calls git::hand_resolved_paths: an uncached rev-list, merge-tree and diff-tree on the raw repo, outside ReadContext''s cache. The bool callers (merge review.current, land''s unreviewed_claim looping approvals per claim, original_approval_valid) never use the paths. They pay that cost, and an error in those git calls now returns Err rather than false. Behaviour is otherwise identical, since None/Some maps exactly to the old true/false. Suggested fix: compute the paths only for the sync caller (for example, return the failing edge and let report_approval call hand_resolved_paths), or route the call through ReadContext''s merge_resolution cache.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.767663522Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bb4bb0d18548d572de9ffbb93054e0a5bfcb63f5
    recorded_at: 2026-10-22T22:34:13.006127472Z
    resolution_commit: 12ffb6f0af9dc064a123b4c405102faa0c991255
    reason: Fixed in 12ffb6f0
    migrated_from: atelier-vrkd
---

low: crates/atelier-app/src/review_gate.rs:206 approval_is_current_to now runs approval_stale_edge, which on every stale merge edge also calls git::hand_resolved_paths: an uncached rev-list, merge-tree and diff-tree on the raw repo, outside ReadContext's cache. The bool callers (merge review.current, land's unreviewed_claim looping approvals per claim, original_approval_valid) never use the paths. They pay that cost, and an error in those git calls now returns Err rather than false. Behaviour is otherwise identical, since None/Some maps exactly to the old true/false. Suggested fix: compute the paths only for the sync caller (for example, return the failing edge and let report_approval call hand_resolved_paths), or route the call through ReadContext's merge_resolution cache.
