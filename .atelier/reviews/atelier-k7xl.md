---
schema: atelier.review
schema_version: 1
id: atelier-k7xl
title: 'Resolved: the advice now says git revert <commit> (or edit back) and then atelier start; base_done_claim only refuses a claim still at the same status at the tip, so the revert withdraws it. The extended test follows the advice end to end (revert, land passes, start succeeds), and replacing the tip condition with true makes that test fail.'
target: atelier-fj1t
identity: Independent reviewer (Claude subagent)
commit: 943c7ee83e3c3e9a1d43a3bb50280b263bf25ef1
created_at: 2026-10-16T18:48:25.526839164Z
event:
  event: disposition
  finding: atelier-l9dd
  decision: fix_now
  supersedes: null
  resolution_commit: 5e071c9c66a28c48db50f62de7c803104fab1021
  follow_up: null
---

Resolved: the advice now says git revert <commit> (or edit back) and then atelier start; base_done_claim only refuses a claim still at the same status at the tip, so the revert withdraws it. The extended test follows the advice end to end (revert, land passes, start succeeds), and replacing the tip condition with true makes that test fail.
