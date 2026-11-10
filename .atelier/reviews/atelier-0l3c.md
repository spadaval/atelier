---
schema: atelier.review
schema_version: 4
id: atelier-0l3c
title: finding on atelier-3kr7 @bc17e990
target: atelier-3kr7
identity: gpt-6.1-sol-3kr7-r1
commit: bc17e990e19366354115bfd698da496c6e2ab695
created_at: 2026-11-10T14:51:15.944232368Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: wont_fix
    identity: gpt-6.1-sol-3kr7-r3
    commit: e73a7dce1de41582fa03d526fbd1aa6565eff1ad
    recorded_at: 2026-11-10T15:02:59.267346978Z
    reason: R1 remains a real ambiguity, accepted under the accountable orchestrator’s original instruction to freeze the rubric after the second rating and document ambiguities in classified.md. I verified that instruction in the original implementation brief. The limits at classified.md:179-185 now state the tracker-decision versus ADR boundary and the rule for a later run; either reading leaves product-defect yield unchanged. Preserve the original rubric and agreement observations for this baseline.
---

medium: docs/architecture/quality/scorecard-baseline/rubric.md:176 R1: The planning override assigns any fault in a decision record to process_nit, but line 195 uses an ADR fault as a wrong_statement example. Sample findings 8 and 67 also label ADR faults wrong_statement. These instructions give a later rater conflicting answers for the same document class, undermining the repeatable baseline. Define the boundary between a planning defect and a false descriptive statement in an ADR, make both examples follow that boundary, and audit the affected labels and reported category totals.
