---
schema: atelier.review
schema_version: 2
id: atelier-m062
title: 'crates/atelier-cli/src/commands/issue.rs:840: issue update --body rewrites Goal/Done when on a done-category record with no trace. Repro: task closed to done with a close reason, then ''issue update <id> --body "## Goal\nOther\n\n## Done when\nTotally different"'' prints ''Updated issue (body)'' and the record keeps status done and its Close Reason under new acceptance criteria. The done claim, evidence and reviews were judged against the old acceptance, and nothing records the change, so after a done claim the acceptance can quietly change before review or merge. Before this change that needed a hand edit visible in a record diff; now it is a supported command. Fix: refuse --body when the status category is done (tell the user to reopen with a transition first), or at least require/append a note recording the change; add a test.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.811472166Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 0f489f2f14870835465001d44cd3f6e024830c81
    recorded_at: 2026-10-15T23:50:09.572486788Z
    resolution_commit: 0f489f2f14870835465001d44cd3f6e024830c81
    reason: Fixed in 0f489f2f14870835465001d44cd3f6e024830c81
    migrated_from: atelier-jy3v
---

crates/atelier-cli/src/commands/issue.rs:840: issue update --body rewrites Goal/Done when on a done-category record with no trace. Repro: task closed to done with a close reason, then 'issue update <id> --body "## Goal\nOther\n\n## Done when\nTotally different"' prints 'Updated issue (body)' and the record keeps status done and its Close Reason under new acceptance criteria. The done claim, evidence and reviews were judged against the old acceptance, and nothing records the change, so after a done claim the acceptance can quietly change before review or merge. Before this change that needed a hand edit visible in a record diff; now it is a supported command. Fix: refuse --body when the status category is done (tell the user to reopen with a transition first), or at least require/append a note recording the change; add a test.
