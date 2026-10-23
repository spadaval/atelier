---
schema: atelier.review
schema_version: 1
id: atelier-2isj
title: finding on atelier-l7nz @e6210fb6
target: atelier-l7nz
identity: Independent reviewer (Claude Sonnet subagent)
commit: e6210fb6abda8f05565720c55d5ce1125e2f69c2
created_at: 2026-10-23T16:20:23.153016901Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/evidence_freshness.rs:215 (item_proof/staleness) judge every receipt against HEAD (fresh=true), but the gates judge a done/merged item's proof historically (claim_endpoint; review_gate historical, doctor). Scenario: item closed, reviewed, merged to main, then any later code commit on main: 'atelier doctor' and 'atelier land' pass ('1 done claim holds') while 'atelier evidence list <id>' marks every pass '⚠ stale (code changed in <later commit>)' and ends 'Proof: ⚠ no fresh pass; newest pass .. is stale'; 'evidence show' likewise. Contradicts u0ig 'same freshness logic as the gates' and docs 'as the gates judge it'; misleads on every finished item viewed from the base. Fix: for done/superseded items judge with the claim-endpoint rule (or show a 'closed' standing instead of stale/no fresh pass), and add a test that merges an item, adds a code commit on base, and compares list with doctor.
