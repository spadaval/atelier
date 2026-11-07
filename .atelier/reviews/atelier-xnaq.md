---
schema: atelier.review
schema_version: 2
id: atelier-xnaq
title: 'crates/atelier-cli/src/commands/issue.rs:840 vs :600: issue create rejects bodies over MAX_ISSUE_DESCRIPTION_LEN (64 KiB, ''Description exceeds maximum length of 65536 bytes'') but issue update --body has no limit. Repro: a 70 KB ''## Goal ... ## Done when'' body is refused by create and accepted by update. Breaks the ''validated like issue create --body'' constraint and lets the size guard be bypassed in two steps. Fix: apply the same length check (shared helper) in update_lifecycle and test it.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.771489956Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 0f489f2f14870835465001d44cd3f6e024830c81
    recorded_at: 2026-10-15T23:50:09.463123430Z
    resolution_commit: 0f489f2f14870835465001d44cd3f6e024830c81
    reason: Fixed in 0f489f2f14870835465001d44cd3f6e024830c81
    migrated_from: atelier-6i6e
---

crates/atelier-cli/src/commands/issue.rs:840 vs :600: issue create rejects bodies over MAX_ISSUE_DESCRIPTION_LEN (64 KiB, 'Description exceeds maximum length of 65536 bytes') but issue update --body has no limit. Repro: a 70 KB '## Goal ... ## Done when' body is refused by create and accepted by update. Breaks the 'validated like issue create --body' constraint and lets the size guard be bypassed in two steps. Fix: apply the same length check (shared helper) in update_lifecycle and test it.
