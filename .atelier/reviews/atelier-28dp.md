---
schema: atelier.review
schema_version: 4
id: atelier-28dp
title: finding on atelier-3kr7 @bc17e990
target: atelier-3kr7
identity: gpt-6.1-sol-3kr7-r1
commit: bc17e990e19366354115bfd698da496c6e2ab695
created_at: 2026-11-10T14:51:24.080594205Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: wont_fix
    identity: gpt-6.1-sol-3kr7-r3
    commit: e73a7dce1de41582fa03d526fbd1aa6565eff1ad
    recorded_at: 2026-11-10T15:03:06.686814759Z
    reason: R2 remains a real first-rater error, accepted under the accountable orchestrator’s original instruction to keep labels frozen after the second rating and answer label disputes in classified.md’s limits. I verified that instruction in the original implementation brief and checked the sampled finding and reviewed item. The limits at classified.md:172-178 explicitly identify entry 38 as process_nit and give the corrected yield, 45 of 100 overall and 7 of 20 for atelier. The published 46 of 100 is expressly the original first rater’s observation. Preserve those observations rather than retroactively changing the baseline labels.
---

medium: docs/architecture/quality/scorecard-baseline/samples/findings.labels.json:155 R2: Sample 38 (atelier-l8sl) is labelled tooling_bug although its main claim is an incomplete CI trigger in the planned item atelier-yn69 and its strategy. At the sampled reviewed commit a9cd1265, the item is todo and the selector has only a Rust job; the proposed web job does not yet exist. This is the rubric lines 174-178 case of a plan describing tooling that would be wrong if built, which must be process_nit. Counting it as tooling_bug makes the published 46 of 100 product-defect yield one too high. Apply the stated planning rule to this entry and recompute the affected repository, category and yield totals in classified.md and the Report; audit the remaining labels for the same planning-versus-implemented distinction.
