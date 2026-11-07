---
schema: atelier.review
schema_version: 2
id: atelier-z96x
title: 'low: crates/atelier-app/src/review_gate.rs:211 and crates/atelier-cli/src/commands/merge_sync.rs:354 Two reporting edge cases. (1) When a clean base merge fails allowed_base_merge only because the base changed an in-scope issue record, hand_resolved_paths is empty, so the report falls back to edge.paths: everything the base brought in, listed as ''Review N paths'' instead of the one record. (2) report_approval drops approvals whose approval_stale_edge errors (Err(_) => {}), so an I/O error turns into ''No accepted approval covers ... yet''. Suggested fix: for (1), list only the in-scope record paths that failed; for (2), surface the error.'
target: atelier-s2ei
identity: Independent reviewer (Claude subagent)
commit: 87eab036a47ab1e9869463512458ac0ebd218a2f
created_at: 2026-10-22T22:28:45.841100891Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bb4bb0d18548d572de9ffbb93054e0a5bfcb63f5
    recorded_at: 2026-10-22T22:34:13.185559977Z
    resolution_commit: 12ffb6f0af9dc064a123b4c405102faa0c991255
    reason: Fixed in 12ffb6f0
    migrated_from: atelier-o99b
---

low: crates/atelier-app/src/review_gate.rs:211 and crates/atelier-cli/src/commands/merge_sync.rs:354 Two reporting edge cases. (1) When a clean base merge fails allowed_base_merge only because the base changed an in-scope issue record, hand_resolved_paths is empty, so the report falls back to edge.paths: everything the base brought in, listed as 'Review N paths' instead of the one record. (2) report_approval drops approvals whose approval_stale_edge errors (Err(_) => {}), so an I/O error turns into 'No accepted approval covers ... yet'. Suggested fix: for (1), list only the in-scope record paths that failed; for (2), surface the error.
