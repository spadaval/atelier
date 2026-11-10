---
schema: atelier.review
schema_version: 2
id: atelier-wvqx
title: finding on atelier-mal9 @e9b51789
target: atelier-mal9
identity: sol-mal9-r1
commit: e9b517897c4be5ce3f4c7904bbbcabec201110da
created_at: 2026-11-10T15:18:58.198410497Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-core/src/review.rs:173 A kept fix-now finding can disappear without being fixed, withdrawn or decided. Reproduced with round 1 finding 1-aaaa.1 (Recommendation: fix now) and round 2 finding 2-bbbb.1 (Was: 1-aaaa.1, Now: kept, Recommendation: defer or no action, verdict: approve): validate returns no problems and ItemDir::open_findings returns an empty list. With Recommendation: fix now on the carrier, validation correctly refuses approval and reports 2-bbbb.1 open. carried_later closes the original for any carrier, while the carrier only stays open when its own recommendation is FixNow. This violates the Done when closure rule and the Records specification that kept findings move and remain open. Reject contradictory kept carriers during validation, or preserve the original fix-now obligation when moving it; add regression tests for both weaker recommendations and approval refusal.
