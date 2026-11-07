---
schema: atelier.review
schema_version: 2
id: atelier-wyj2
title: 'touched_claims resolves each changed evidence path through the HEAD index only. A deleted evidence record (index.evidence is None) or one whose target and attachments were edited to another issue no longer names the issue it used to prove, so that claim is skipped. Reproduced on the base with an upstream: claim with a fail then a pass; git rm the pass (or sed its target to another issue), commit. atelier check and land pass; doctor reports ''evidence.no_newer_failure: newer failing evidence''. Fix: for every changed path, also parse the record at the trusted point and take the union of its targets before and after (same for reviews, for consistency). Add both cases to the land test.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6039144f2ace03f72f3112f5c4f01afa5cfce4f7
created_at: 2026-10-16T23:29:50.993076885Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: cd12e2b9b4f09faad6173599625dddb8e326a8de
    recorded_at: 2026-10-16T23:38:02.704670654Z
    resolution_commit: ebdc2ad5f9a38bdb0a17192c7da36fdd469ee51f
    reason: Changed evidence and review records touch their targets as they were at the trusted point and as they are; a test removes passing proof
    migrated_from: atelier-socw
---

touched_claims resolves each changed evidence path through the HEAD index only. A deleted evidence record (index.evidence is None) or one whose target and attachments were edited to another issue no longer names the issue it used to prove, so that claim is skipped. Reproduced on the base with an upstream: claim with a fail then a pass; git rm the pass (or sed its target to another issue), commit. atelier check and land pass; doctor reports 'evidence.no_newer_failure: newer failing evidence'. Fix: for every changed path, also parse the record at the trusted point and take the union of its targets before and after (same for reviews, for consistency). Add both cases to the land test.
