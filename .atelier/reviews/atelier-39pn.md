---
schema: atelier.review
schema_version: 2
id: atelier-39pn
title: finding on atelier-fcb7 @ea1d42a1
target: atelier-fcb7
identity: gpt-6-astra-amend2-r1
commit: ea1d42a10dc41d45403d26629567c8564335f5a0
created_at: 2026-11-10T03:59:50.175399917Z
event:
  event: finding
  recommendation: fix_now
---

medium, docs/strategies/redesign-product.md:289 and redesign-architecture.md:137,191,255: settlement is required on the base, but the product says a started item directory changes only on its branch, and the generic work-branch command rule writes that branch. The settlement act requires only a verdict, so it also accepts a review that exists only on an unmerged work branch and cannot be settled on the base. Workers following these rules can refuse the intended done-item operation or create divergent settlements. The hooks themselves permit a valid record-only settlement with unchanged status; they do not resolve this ownership conflict. Smallest fix: explicitly exempt settlements from the product branch-only rule, define base-only lookup/write or refusal from other checkouts, and require the reviewed finding to exist on the base (or restrict settlement to done items); test second settlement refusal against that authoritative copy.
