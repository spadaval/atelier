---
schema: atelier.review
schema_version: 1
id: atelier-h7el
title: 'crates/atelier-cli/src/commands/session.rs:482: the behind-base note goes into ''lines'', which is thrown away when land bails with land_invariants_unmet. So a branch behind its base that also fails a gate (seen: missing close_reason) prints only the gate failures and never says the base moved. Fixing those gates and rerunning shows the note, so the impact is low. Consider including the note in the failure text.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.077590021Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:482: the behind-base note goes into 'lines', which is thrown away when land bails with land_invariants_unmet. So a branch behind its base that also fails a gate (seen: missing close_reason) prints only the gate failures and never says the base moved. Fixing those gates and rerunning shows the note, so the impact is low. Consider including the note in the failure text.
