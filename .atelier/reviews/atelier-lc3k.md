---
schema: atelier.review
schema_version: 2
id: atelier-lc3k
title: finding on atelier-l7nz @1d23aab7
target: atelier-l7nz
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: 1d23aab77b622c215fbb69573237a52dfa28b73d
created_at: 2026-10-23T16:31:27.227853740Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 7e6c27de7f763e2868b6f81f83604d96e39b8357
    recorded_at: 2026-10-23T16:31:50.931400341Z
    follow_up: atelier-4urj
    reason: No gate judges a superseded item's proof, so nothing is contradicted; unify with the shared supersession rule in atelier-4urj.
    migrated_from: atelier-mmw7
---

low: crates/atelier-app/src/evidence_freshness.rs:248 judged_at_close excludes superseded items, so after merge plus a later base code commit 'evidence list' on a superseded item marks its pass 'stale (code changed in ...)' and the Proof line 'no fresh pass', while land and doctor pass. No gate judges a superseded item's proof (evidence.attached is a close validator), so nothing contradicts a gate, but the stale warning is noise on retired work; consider no proof verdict for superseded items (could ride with atelier-4urj).
