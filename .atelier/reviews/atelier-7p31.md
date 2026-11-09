---
schema: atelier.review
schema_version: 2
id: atelier-7p31
title: finding on atelier-zpmb @2580f29e
target: atelier-zpmb
identity: sol-plan-reviewer (gpt-6.1-sol)
commit: 2580f29e70a63df886f976fd663588960eb65b11
created_at: 2026-11-09T06:50:35.600480707Z
event:
  event: finding
  recommendation: fix_now
---

medium: Pin criteria and receipt evaluation as well as record loading. The strategy's Design/Derisking (fleet-dashboard.md:83-92,145-148) reduces the work to a loader and two strategy read sites, but criteria::mission_criteria also reads the working-tree strategy (crates/atelier-app/src/criteria.rs:48), and mission_proof_until runs rev-parse HEAD directly (:249), bypassing ReadContext::from_head. Against the common Git directory this is the main checkout's HEAD, which can differ from the base commit. Thus the proposed strategy-text fixture can pass while receipt marks and ages come from another branch and are cached under an unchanged base commit. Expand the design and this ready item's proof to pass one captured base commit through criteria/history evaluation and load policy/config/strategy from that snapshot. Test different criteria and receipt ancestry with common-directory HEAD on a divergent or behind branch; assert criteria, marks, ages and generated_from, not only strategy prose.
