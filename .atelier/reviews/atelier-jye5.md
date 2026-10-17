---
schema: atelier.review
schema_version: 1
id: atelier-jye5
title: 'crates/atelier-app/src/criteria.rs:101,231 (and lint.rs:320 via receipt_criteria_problems): the dqty leak survives re-scoping. Which mission a receipt''s criteria belong to is worked out from the target''s scope at read time, not stored when the receipt is recorded. So if an item moves from mission A to mission B (issue unlink A, issue link B), its A proof counts toward B by slug. Reproduced in a scratch repo: receipt atelier-kl20 recorded S1=pass on an item in mission A (''First of A''); after the move, ''work mission B'' showed ''S1 ✓ proven First of B. | atelier-kl20 pass'' and ''Criteria: 1 criterion: 1 proven'', and ''check'' passed. Positional S<n> slugs are shared by every strategy, so this mis-attribution is silent whenever a moved item carries proof. Reparenting an item into another mission''s scope does the same. Fix: store the owning mission id on the receipt''s criteria when it is recorded (the target''s single mission), match proof and check against that stored id, and report in check a receipt whose stored mission no longer holds its target. Doing this now, while few receipts name criteria, avoids migrating them later, before debrief (atelier-rx22) builds on mission_proof.'
target: atelier-ho7o
identity: Independent reviewer 2 (Claude subagent)
commit: 97f4dc47ec4ddf96e4da406883b0780a6da947d8
created_at: 2026-10-17T18:47:41.212702041Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/criteria.rs:101,231 (and lint.rs:320 via receipt_criteria_problems): the dqty leak survives re-scoping. Which mission a receipt's criteria belong to is worked out from the target's scope at read time, not stored when the receipt is recorded. So if an item moves from mission A to mission B (issue unlink A, issue link B), its A proof counts toward B by slug. Reproduced in a scratch repo: receipt atelier-kl20 recorded S1=pass on an item in mission A ('First of A'); after the move, 'work mission B' showed 'S1 ✓ proven First of B. | atelier-kl20 pass' and 'Criteria: 1 criterion: 1 proven', and 'check' passed. Positional S<n> slugs are shared by every strategy, so this mis-attribution is silent whenever a moved item carries proof. Reparenting an item into another mission's scope does the same. Fix: store the owning mission id on the receipt's criteria when it is recorded (the target's single mission), match proof and check against that stored id, and report in check a receipt whose stored mission no longer holds its target. Doing this now, while few receipts name criteria, avoids migrating them later, before debrief (atelier-rx22) builds on mission_proof.
