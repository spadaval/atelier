---
schema: atelier.review
schema_version: 2
id: atelier-h7el
title: 'crates/atelier-cli/src/commands/session.rs:482: the behind-base note goes into ''lines'', which is thrown away when land bails with land_invariants_unmet. So a branch behind its base that also fails a gate (seen: missing close_reason) prints only the gate failures and never says the base moved. Fixing those gates and rerunning shows the note, so the impact is low. Consider including the note in the failure text.'
target: atelier-qz9x
identity: Independent reviewer (Claude subagent)
commit: 74615395876922a66dd99e748feb2a964ee26f79
created_at: 2026-10-16T18:32:13.077590021Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 5c2e3b0e7fec9f639eb9cfc08cb680b54925c115
    recorded_at: 2026-10-16T18:41:55.071524009Z
    resolution_commit: 25a5eb219903546052b8a4425bc0f4edcc87bd11
    reason: 'Resolved: session.rs builds behind_note once, before the gates run, and adds it to the land_invariants_unmet error. The behind test shows a failing review.current gate whose output includes git merge main.'
    migrated_from: atelier-ny4g
---

crates/atelier-cli/src/commands/session.rs:482: the behind-base note goes into 'lines', which is thrown away when land bails with land_invariants_unmet. So a branch behind its base that also fails a gate (seen: missing close_reason) prints only the gate failures and never says the base moved. Fixing those gates and rerunning shows the note, so the impact is low. Consider including the note in the failure text.
