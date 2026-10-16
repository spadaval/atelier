---
schema: atelier.review
schema_version: 1
id: atelier-q9a9
title: 'Follow-up to atelier-5krb: the mission exemption is now tested (removing "mission" from the match at git.rs:337 fails the test), but the merge-commit rule is not. The pull merge in base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions (tests/session.rs ~2255-2280) is a clean merge, so check_base_commits skips it at git.rs:247 before base_done_claim runs. Changing parents.iter().all to parents.iter().take(1).all (first parent only) still passes the test. A conflicting pull merge, resolved so that the record takes the upstream''s done version and differs from both parents, would cover the all-parents comparison.'
target: atelier-fj1t
identity: Independent reviewer (Claude subagent)
commit: 943c7ee83e3c3e9a1d43a3bb50280b263bf25ef1
created_at: 2026-10-16T18:48:25.657602291Z
event:
  event: finding
  recommendation: defer
---

Follow-up to atelier-5krb: the mission exemption is now tested (removing "mission" from the match at git.rs:337 fails the test), but the merge-commit rule is not. The pull merge in base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions (tests/session.rs ~2255-2280) is a clean merge, so check_base_commits skips it at git.rs:247 before base_done_claim runs. Changing parents.iter().all to parents.iter().take(1).all (first parent only) still passes the test. A conflicting pull merge, resolved so that the record takes the upstream's done version and differs from both parents, would cover the all-parents comparison.
