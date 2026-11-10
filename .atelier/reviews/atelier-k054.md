---
schema: atelier.review
schema_version: 2
id: atelier-k054
title: finding on atelier-fcb7 @8f1d6bfc
target: atelier-fcb7
identity: grok-4.6-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:37:23.771590037Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-product.md:101 versus redesign-architecture.md:174: the product lifecycle says approved leaves only when merge runs, while the act table has submit from approved and atelier-5cuc tests a stale approval. An implementer following the product table will not resubmit a stale head. Smallest fix: approved also leaves when submit runs because the approval no longer covers the head.
