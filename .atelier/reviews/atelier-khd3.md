---
schema: atelier.review
schema_version: 1
id: atelier-khd3
title: 'low: crates/atelier-cli/tests/session.rs:4279 ''assert!(!status.contains("active)"))'' is vacuous: no view prints ''active)'', so it cannot fail and replaced the meaningful ''| 0 active'' check. Fix: assert the done branch''s Branches row is exactly ''<branch>  <id> done, merged  <title>'' (e.g. a full-line equality on the row) or delete the assertion.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.695296331Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/tests/session.rs:4279 'assert!(!status.contains("active)"))' is vacuous: no view prints 'active)', so it cannot fail and replaced the meaningful '| 0 active' check. Fix: assert the done branch's Branches row is exactly '<branch>  <id> done, merged  <title>' (e.g. a full-line equality on the row) or delete the assertion.
