---
schema: atelier.review
schema_version: 2
id: atelier-xqjc
title: finding on atelier-secy @6e13df06
target: atelier-secy
identity: lighter-workflow reviewer for atelier-secy (claude-opus-5-5)
commit: 6e13df06a8b569040428bea5aca026c27bdb76ed
created_at: 2026-11-07T03:48:40.588672913Z
event:
  event: finding
  recommendation: defer
---

Medium: crates/atelier-app/src/evidence_freshness.rs:136 first_mismatch still compares the whole Done when (acceptance_matches against the HEAD record at :136 and per edge at :145), so after a validation tick on the base the item's receipts are AcceptanceChanged in every path, including held_at_close and proof_matches(fresh=false). Confirmed: a policy with 'evidence.attached: { min_count: 1 }' on work close, item closed with a pass, approved, merged, then '- [x] After merge' ticked on main: land refuses 'fx-... evidence.attached: expected at least 1 fresh passing evidence record; found 0'. So the after-merge path the Goal promises for every merged item still refuses under an opt-in evidence gate (not starter or this repo's policy; not new). Fix: compare with the same-claim rule (only_validated, given the index) in both first_mismatch comparisons; file a follow-up bug with this reproduction.
