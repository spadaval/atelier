---
schema: atelier.review
schema_version: 1
id: atelier-wyj2
title: 'touched_claims resolves each changed evidence path through the HEAD index only. A deleted evidence record (index.evidence is None) or one whose target and attachments were edited to another issue no longer names the issue it used to prove, so that claim is skipped. Reproduced on the base with an upstream: claim with a fail then a pass; git rm the pass (or sed its target to another issue), commit. atelier check and land pass; doctor reports ''evidence.no_newer_failure: newer failing evidence''. Fix: for every changed path, also parse the record at the trusted point and take the union of its targets before and after (same for reviews, for consistency). Add both cases to the land test.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6039144f2ace03f72f3112f5c4f01afa5cfce4f7
created_at: 2026-10-16T23:29:50.993076885Z
event:
  event: finding
  recommendation: fix_now
---

touched_claims resolves each changed evidence path through the HEAD index only. A deleted evidence record (index.evidence is None) or one whose target and attachments were edited to another issue no longer names the issue it used to prove, so that claim is skipped. Reproduced on the base with an upstream: claim with a fail then a pass; git rm the pass (or sed its target to another issue), commit. atelier check and land pass; doctor reports 'evidence.no_newer_failure: newer failing evidence'. Fix: for every changed path, also parse the record at the trusted point and take the union of its targets before and after (same for reviews, for consistency). Add both cases to the land test.
