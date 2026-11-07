---
schema: atelier.review
schema_version: 2
id: atelier-khd3
title: 'low: crates/atelier-cli/tests/session.rs:4279 ''assert!(!status.contains("active)"))'' is vacuous: no view prints ''active)'', so it cannot fail and replaced the meaningful ''| 0 active'' check. Fix: assert the done branch''s Branches row is exactly ''<branch>  <id> done, merged  <title>'' (e.g. a full-line equality on the row) or delete the assertion.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.695296331Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: bca612b06974a12e3d5b1a73775266b03c9c0523
    recorded_at: 2026-10-22T20:47:02.319638231Z
    resolution_commit: 1d2f67df28ea9dfd13863c5f41e37302f069dbba
    reason: Replaced the vacuous check with a full-row assertion.
    migrated_from: atelier-bhna
---

low: crates/atelier-cli/tests/session.rs:4279 'assert!(!status.contains("active)"))' is vacuous: no view prints 'active)', so it cannot fail and replaced the meaningful '| 0 active' check. Fix: assert the done branch's Branches row is exactly '<branch>  <id> done, merged  <title>' (e.g. a full-line equality on the row) or delete the assertion.
