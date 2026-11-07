---
schema: atelier.review
schema_version: 2
id: atelier-nlvx
title: 'low: crates/atelier-cli/tests/merge_gates.rs:3184-3216 (sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval). The ns03 agreement test runs ''merge --check'' before the sync and asserts only that it fails (it could fail for any gate), then asserts the sync report says stale; it never checks that review.current fails after the sync, which was the reported disagreement. A scratch probe confirms agreement after the sync under both strategies with --no-ff and --ff-only landings, plus a no-new-code variant and a fresh-approval variant, so the code is right but the test would not catch a regression in which gate fails. Fix: after the sync, run ''merge --check --json'' and assert the review.current node''s passed equals !stale.'
target: atelier-fngz
identity: Independent reviewer (Claude subagent)
commit: 97e785a471c9a95cea99441e15682e56a1c90826
created_at: 2026-10-22T23:43:27.308174043Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: ef59b1109fdeb322bba48bdded0f8a16981360c1
    recorded_at: 2026-10-22T23:45:57.341809428Z
    resolution_commit: 5959474657df56a8fd49bab0b7215c7fb6a05268
    reason: Fixed in 59594746
    migrated_from: atelier-586j
---

low: crates/atelier-cli/tests/merge_gates.rs:3184-3216 (sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval). The ns03 agreement test runs 'merge --check' before the sync and asserts only that it fails (it could fail for any gate), then asserts the sync report says stale; it never checks that review.current fails after the sync, which was the reported disagreement. A scratch probe confirms agreement after the sync under both strategies with --no-ff and --ff-only landings, plus a no-new-code variant and a fresh-approval variant, so the code is right but the test would not catch a regression in which gate fails. Fix: after the sync, run 'merge --check --json' and assert the review.current node's passed equals !stale.
