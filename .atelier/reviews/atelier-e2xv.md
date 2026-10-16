---
schema: atelier.review
schema_version: 1
id: atelier-e2xv
title: 'An out-of-scope record resolution can change the owner''s readiness. allowed_resolution (crates/atelier-app/src/review_gate.rs:192-198) passes any out-of-scope issue record, including its relationships.blocks and its status, and ''blocks'' is stored on the blocker, not on the owner. Reproduced: the base records an open issue Z that blocks the owner, so blockers.none_open FAILs. A later base merge commit runs ''atelier issue unlink Z <owner> --role blocks''. Afterwards blockers.none_open and review.current both pass with no re-review. The no-conflict route is the first finding; the same holds whenever Z''s record really conflicts. Fix: a resolution of an out-of-scope record must not add, remove or change relationships that target an id in scope. Better still, accept only a resolution that keeps each side''s status and relationships, for example the union of the two sides as in the astromech advances case.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: 961e69d6cffe8612e03d743f7efcf5a158dec220
created_at: 2026-10-16T19:55:05.589222328Z
event:
  event: finding
  recommendation: fix_now
---

An out-of-scope record resolution can change the owner's readiness. allowed_resolution (crates/atelier-app/src/review_gate.rs:192-198) passes any out-of-scope issue record, including its relationships.blocks and its status, and 'blocks' is stored on the blocker, not on the owner. Reproduced: the base records an open issue Z that blocks the owner, so blockers.none_open FAILs. A later base merge commit runs 'atelier issue unlink Z <owner> --role blocks'. Afterwards blockers.none_open and review.current both pass with no re-review. The no-conflict route is the first finding; the same holds whenever Z's record really conflicts. Fix: a resolution of an out-of-scope record must not add, remove or change relationships that target an id in scope. Better still, accept only a resolution that keeps each side's status and relationships, for example the union of the two sides as in the astromech advances case.
